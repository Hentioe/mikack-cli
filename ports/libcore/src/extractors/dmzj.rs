use super::{prelude::*, *};
use crate::{errors::*, html, http, jsrt, models::*};
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

pub struct Dmzj;

impl Extractor for Dmzj {
    fn index(&self, more: u32) -> Result<Vec<Detail>> {
        let url = if more > 0 {
            format!(
                "https://manhua.dmzj.com/rank/total-block-{}.shtml",
                more + 1
            )
        } else {
            "https://manhua.dmzj.com/rank/".to_string()
        };
        let fll: LinkListConverter<Detail> =
            LinkListConverter::new(&url, ".middlerighter span.title > a", vec![]);
        fll.try_get_list()?.result()
    }

    fn fetch_sections(&self, detail: &mut Detail) -> Result<()> {
        let url = &detail.url;
        let mut helper = http::SendHelper::new();
        helper.send_get(url)?;

        match helper.result() {
            http::Result::Ok(html_s) => {
                let doc = html::parse_document(&html_s);
                let title_e = doc
                    .select(&html::parse_select(".anim_title_text > a > h1")?)
                    .next()
                    .ok_or(err_msg(format!("no title found, {}", &detail.url)))?;
                let title = title_e.text().next().ok_or(err_msg(format!(
                    "no title found, {}",
                    &title_e.inner_html()
                )))?;
                for element in doc.select(&html::parse_select(
                    ".middleright_mr > div > ul > li > a[title]",
                )?) {
                    let sec = Section::new(
                        &format!(
                            "{} {}",
                            &title,
                            element.text().next().ok_or(err_msg(format!(
                                "no text found, {}",
                                element.inner_html()
                            )),)?
                        ),
                        &format!(
                            "{}{}",
                            "https://manhua.dmzj.com",
                            element.value().attr("href").ok_or(err_msg(format!(
                                "no href found, {}",
                                element.inner_html()
                            )))?
                        ),
                    );
                    detail.add_section(sec);
                }
                Ok(())
            }
            http::Result::Err(e) => Err(e),
        }
    }

    fn fetch_pages(&self, section: &mut Section) -> Result<()> {
        let mut helper = http::SendHelper::new();
        helper.send_get(&section.url)?;

        match helper.result() {
            http::Result::Ok(html_s) => {
                // 抽取并包装加密代码块
                let caps = RE_CRYPTO_CODE
                    .captures(&html_s)
                    .ok_or(err_msg("no encrypted code block found"))?;
                let code = caps
                    .get(1)
                    .ok_or(err_msg("no encrypted code block found"))?
                    .as_str();
                let wrapper_code = format!("{}\n{}", &code, "console.log(JSON.stringify({pages: eval(pages), name: `${g_comic_name} ${g_chapter_name}`}));");
                // 托管给 JSRT 并获取结果
                let output = jsrt::read_output(&wrapper_code)?;
                let v: Value = serde_json::from_str(&output)?;
                if !section.has_name() {
                    section.name = v["name"]
                        .as_str()
                        .ok_or(err_msg(format!(
                            "did not get the name string through JSON conversion, {}",
                            &section.url
                        )))?
                        .to_string();
                }
                let pages = v["pages"].as_array().ok_or(err_msg(format!(
                    "did not get the pages list through JSON conversion, {}",
                    &section.url
                )))?;
                for (i, v) in pages.iter().enumerate() {
                    let url = format!(
                        "{}{}",
                        "https://images.dmzj.com/",
                        v.as_str().ok_or(err_msg(format!(
                            "did not get the url string through JSON conversion, {}",
                            &section.url
                        )))?
                    );
                    section.add_page(Page::new(i as u32, &url));
                }
                Ok(())
            }
            http::Result::Err(e) => Err(e),
        }
    }
}

lazy_static! {
    static ref RE_CRYPTO_CODE: Regex =
        Regex::new(r#"<script type="text/javascript">([\s\S]+)var res_type"#).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dmzj_index() {
        let det_list = Dmzj {}.index(0).unwrap();
        assert_eq!(20, det_list.len());
    }

    #[test]
    fn test_dmzj_fetch_sections() {
        let mut datail = Detail::new("一拳超人", "https://manhua.dmzj.com/yiquanchaoren/");
        Dmzj {}.fetch_sections(&mut datail).unwrap();
        assert_eq!(376, datail.section_list.len());
    }

    #[test]
    fn test_dmzj_fetch_pages() {
        let mut section = Section::new(
            "一拳超人 第142话",
            "https://manhua.dmzj.com/yiquanchaoren/80709.shtml#@page=1",
        );
        Dmzj {}.fetch_pages(&mut section).unwrap();
        assert_eq!(28, section.page_list.len());
    }
}
