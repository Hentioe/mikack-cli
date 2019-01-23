use super::{prelude::*, *};
use crate::{errors::*, html, http, models::*};
use encoding_rs::*;

pub struct Dmk;

impl Extractor for Dmk {
    fn index(&self, more: u32) -> Result<Vec<Detail>> {
        let url = if more > 0 {
            let more = more + 1;
            let page_s = if more < 10 {
                format!("0{}", more)
            } else {
                more.to_string()
            };
            format!("http://www.hhmmoo.com/comic/{}.html", page_s)
        } else {
            "https://www.cartoonmad.com/endcm.html".to_string()
        };

        let mut fll: LinkListConverter<Detail> =
            LinkListConverter::new(&url, "td[colspan=\"2\"] td[align=\"center\"] > a", vec![]);
        fll.set_href_prefix("http://www.cartoonmad.com/")
            .set_encoding(BIG5);
        let list = fll.try_get_list()?.list;
        Ok(list[0..list.len() - 6].to_vec())
    }

    fn fetch_sections(&self, detail: &mut Detail) -> Result<()> {
        let url = &detail.url;
        let mut fll: LinkListConverter<Section> =
            LinkListConverter::new(&url, "fieldset td > a", vec![]);
        fll.set_href_prefix("http://www.cartoonmad.com")
            .set_encoding(BIG5)
            .text_prefix_finder(&|doc| {
                let name = html::find_text(doc, "title")?
                    .trim()
                    .replace(" - 免費漫畫區 - 動漫狂", "")
                    .to_string();
                Ok(name)
            });
        let section_list = fll.try_get_list()?.result()?;
        detail.section_list = section_list;
        Ok(())
    }

    fn fetch_pages(&self, section: &mut Section) -> Result<()> {
        let mut helper = http::SendHelper::new();
        helper.send_get(&section.url)?;

        match helper.result_bytes() {
            http::RawResult::Err(e) => Err(e),
            http::RawResult::Ok(resp_bytes) => {
                let (cow, _encoding_used, had_errors) = BIG5.decode(&resp_bytes);
                if had_errors {
                    return Err(err_msg(format!(
                        "character encoding conversion failed, {}",
                        &section.url
                    )));
                }
                let doc = html::parse_document(&cow[..]);
                let mut list: Vec<String> = vec![];
                for element in doc.select(&html::parse_select(
                    "select[name=\"jump\"] > option[value]",
                )?) {
                    list.push(format!(
                        "{}{}",
                        "http://www.cartoonmad.com/comic/",
                        element
                            .value()
                            .attr("value")
                            .ok_or(err_msg(format!(
                                "did not get a list of pages, {}",
                                &section.url
                            )))?
                            .to_owned()
                    ));
                }
                if !section.has_name() {
                    section.name = doc
                        .select(&html::parse_select("title")?)
                        .next()
                        .ok_or(err_msg(format!(
                            "did not get the page title, {}",
                            &section.url
                        )))?
                        .text()
                        .next()
                        .ok_or(err_msg(format!(
                            "did not get the page title, {}",
                            &section.url
                        )))?
                        .trim()
                        .replace(" - 動漫狂", "")
                        .to_string();
                }
                for (i, p_url) in list.iter().enumerate() {
                    let mut helper = http::SendHelper::new();
                    helper.send_get(&p_url)?;
                    match helper.result_bytes() {
                        http::RawResult::Ok(resp_bytes) => {
                            let (cow, _encoding_used, had_errors) = BIG5.decode(&resp_bytes);
                            if had_errors {
                                return Err(err_msg(format!(
                                    "character encoding conversion failed, {}",
                                    &section.url
                                )));
                            }
                            let doc = html::parse_document(&cow[..]);
                            let url = doc
                                .select(&html::parse_select(
                                    "a > img[oncontextmenu='return false']",
                                )?)
                                .next()
                                .ok_or(err_msg(format!("no image found, {}", &p_url)))?
                                .value()
                                .attr("src")
                                .ok_or(err_msg(format!("no image found, {}", &p_url)))?;
                            section.add_page(Page::new(
                                i as u32,
                                &format!("{}{}", "http://www.cartoonmad.com", &url),
                            ));
                        }
                        http::RawResult::Err(e) => return Err(e),
                    }
                }
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dmk_index() {
        let det_list = Dmk {}.index(0).unwrap();
        assert_eq!(60, det_list.len());
    }

    #[test]
    fn test_dmk_fetch_sections() {
        let mut detail = Detail::new("魔導少年", "https://www.cartoonmad.com/comic/1153.html");
        Dmk {}.fetch_sections(&mut detail).unwrap();
        assert_eq!(411, detail.section_list.len());
    }

    #[test]
    fn test_dmk_fetch_pages() {
        let mut section = Section::new(
            "魔導少年 第 153 話",
            "https://www.cartoonmad.com/comic/115301532018001.html",
        );
        Dmk {}.fetch_pages(&mut section).unwrap();
        assert_eq!(18, section.page_list.len());
    }
}
