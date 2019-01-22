use crate::{
    errors::*,
    fetch::{html, http, prelude::*, *},
};
use encoding_rs::*;

pub struct Dmk;

impl Extractor for Dmk {
    fn index(&self, more: u32) -> Result<Vec<Detail>> {
        let mut det_list: Vec<Detail> = vec![];
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
        let mut helper = http::SendHelper::new();
        helper.send_get(&url)?;
        match helper.result_bytes() {
            http::RawResult::Ok(resp_bytes) => {
                let (cow, _encoding_used, had_errors) = BIG5.decode(&resp_bytes);
                if had_errors {
                    return Err(err_msg(format!(
                        "character encoding conversion failed, {}",
                        &url
                    )));
                }
                let doc = html::parse_document(&cow[..]);
                for element in doc.select(&html::parse_select(
                    "td[colspan=\"2\"] td[align=\"center\"] > a",
                )?) {
                    let text = element
                        .text()
                        .next()
                        .ok_or(err_msg(format!("no text found, {}", element.inner_html())))?;
                    let det = Detail::new(
                        text,
                        &format!(
                            "{}{}",
                            "http://www.cartoonmad.com/",
                            element.value().attr("href").ok_or(err_msg(format!(
                                "no href found, {}",
                                element.inner_html()
                            )))?
                        ),
                    );
                    det_list.push(det);
                }
                Ok(det_list[0..(det_list.len() - 6)].to_vec())
            }
            http::RawResult::Err(e) => Err(e),
        }
    }

    fn fetch_sections(&self, detail: &mut Detail) -> Result<()> {
        let url = &detail.url;
        let mut helper = http::SendHelper::new();
        helper.send_get(url)?;

        match helper.result_bytes() {
            http::RawResult::Ok(resp_bytes) => {
                let (cow, _encoding_used, had_errors) = BIG5.decode(&resp_bytes);
                if had_errors {
                    return Err(err_msg(format!(
                        "character encoding conversion failed, {}",
                        &url
                    )));
                }
                let doc = html::parse_document(&cow[..]);
                let name = doc
                    .select(&html::parse_select("title")?)
                    .next()
                    .ok_or(err_msg(format!("did not get the page title, {}", &url)))?
                    .text()
                    .next()
                    .ok_or(err_msg(format!("did not get the page title, {}", &url)))?
                    .trim()
                    .replace(" - 免費漫畫區 - 動漫狂", "")
                    .to_string();
                for element in doc.select(&html::parse_select("fieldset td > a")?) {
                    let sec = Section::new(
                        &format!(
                            "{} {}",
                            &name,
                            element.text().next().ok_or(err_msg(format!(
                                "no text found, {}",
                                element.inner_html()
                            )))?
                        ),
                        &format!(
                            "{}{}",
                            "http://www.cartoonmad.com",
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
            http::RawResult::Err(e) => Err(e),
        }
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
