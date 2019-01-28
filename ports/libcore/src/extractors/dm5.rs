use super::{prelude::*, *};
use crate::{errors::*, html, http, /*jsrt,*/ models::*};
use lazy_static::lazy_static;
use regex::Regex;
use url::percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};
//use serde_json::Value;

pub struct Dm5;

impl Extractor for Dm5 {
    fn index(&self, _more: u32) -> Result<Vec<Detail>> {
        let url = "http://www.dm5.com/manhua-rank/?t=2";
        let mut fll: LinkListConverter<Detail> = LinkListConverter::new(
            url,
            "ul.mh-list.col3.top-cat > li .mh-item-detali > h2.title > a",
            vec![],
        );
        fll.set_href_prefix("http://www.dm5.com");
        fll.try_get_list()?.result()
    }

    fn fetch_sections(&self, detail: &mut Detail) -> Result<()> {
        let url = &detail.url;
        let mut fll: LinkListConverter<Section> =
            LinkListConverter::new(&url, "ul > li > a[title]", vec![]);
        fll.set_href_prefix("http://www.dm5.com")
            .text_prefix_finder(&|doc| {
                let name = html::find_text(doc, ".banner_detail_form > .info > p.title")?.trim();
                Ok(name.to_string())
            });
        let section_list = fll.try_get_list()?.result()?;
        detail.section_list = section_list;
        Ok(())
    }

    fn fetch_pages(&self, section: &mut Section) -> Result<()> {
        let mut helper = http::SendHelper::new();
        helper.send_get(&section.url)?;

        match helper.result() {
            http::Result::Ok(html_s) => {
                let _name = RE_NAME
                    .captures(&html_s)
                    .ok_or(err_msg("no name variable found"))?
                    .get(1)
                    .ok_or(err_msg("no name found"))?
                    .as_str();
                let mid = RE_MID
                    .captures(&html_s)
                    .ok_or(err_msg("no mid variable found"))?
                    .get(1)
                    .ok_or(err_msg("no mid found"))?
                    .as_str();
                let cid = RE_CID
                    .captures(&html_s)
                    .ok_or(err_msg("no cid variable found"))?
                    .get(1)
                    .ok_or(err_msg("no cid found"))?
                    .as_str();
                let _count = RE_COUNT
                    .captures(&html_s)
                    .ok_or(err_msg("no count variable found"))?
                    .get(1)
                    .ok_or(err_msg("no count found"))?
                    .as_str();
                let sign = RE_SIGN
                    .captures(&html_s)
                    .ok_or(err_msg("no sign variable found"))?
                    .get(1)
                    .ok_or(err_msg("no sign found"))?
                    .as_str();
                let dt = RE_DT
                    .captures(&html_s)
                    .ok_or(err_msg("no dt variable found"))?
                    .get(1)
                    .ok_or(err_msg("no dt found"))?
                    .as_str();
                let _url_params = format!(
                    "&cid={}&_cid={}&_mid={}&_dt={}&_sign={}",
                    cid,
                    cid,
                    mid,
                    utf8_percent_encode(dt, DEFAULT_ENCODE_SET).to_string(),
                    sign
                );

                Ok(())
            }
            http::Result::Err(e) => Err(e),
        }
    }
}

lazy_static! {
    static ref RE_NAME: Regex = Regex::new(r#"var\s*DM5_CTITLE\s*=\s*"([^"]+)";"#).unwrap();
    static ref RE_MID: Regex = Regex::new(r#"var\s*DM5_MID\s*=\s*([^;]+);"#).unwrap();
    static ref RE_CID: Regex = Regex::new(r#"var\s*DM5_CID\s*=\s*([^;]+);"#).unwrap();
    static ref RE_COUNT: Regex = Regex::new(r#"var\s*DM5_IMAGE_COUNT\s*=\s*([^;]+);"#).unwrap();
    static ref RE_SIGN: Regex = Regex::new(r#"var\s*DM5_VIEWSIGN\s*=\s*"([^"]+)";"#).unwrap();
    static ref RE_DT: Regex = Regex::new(r#"var\s*DM5_VIEWSIGN_DT\s*=\s*"([^"]+)";"#).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dm5_index() {
        let list = Dm5 {}.index(0).unwrap();
        assert_eq!(297, list.len());
    }

    #[test]
    fn test_dm5_fetch_sections() {
        let mut detail = Detail::new(
            "灵能百分百",
            "http://www.dm5.com/manhua-lingnengbaifenbai/",
        );
        Dm5 {}.fetch_sections(&mut detail).unwrap();
        assert_eq!(283, detail.section_list.len());
    }

    //        #[test]
    //        fn test_dm5_fetch_pages() {
    //            let mut section = Section::new(
    //                UNKNOWN_NAME,
    //                "http://www.dm5.com/m765383/",
    //            );
    //            Dm5 {}.fetch_pages(&mut section).unwrap();
    //            assert_eq!(21, section.page_list.len());
    //        }
}
