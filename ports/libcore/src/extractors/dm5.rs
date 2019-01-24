use super::{prelude::*, *};
use crate::{errors::*, html, http, /*jsrt,*/ models::*};
//use lazy_static::lazy_static;
//use regex::Regex;
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
            http::Result::Ok(_html_s) => Ok(()),
            http::Result::Err(e) => Err(e),
        }
    }
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
        let mut detail = Detail::new("一拳超人", "http://www.dm5.com/manhua-yiquanchaoren/");
        Dm5 {}.fetch_sections(&mut detail).unwrap();
        assert_eq!(350, detail.section_list.len());
    }

    //    #[test]
    //    fn test_dm5_fetch_pages() {
    //        let mut section = Section::new(
    //            UNKNOWN_NAME,
    //            "http://www.dm5.com/m765383/",
    //        );
    //        Dm5 {}.fetch_pages(&mut section).unwrap();
    //        assert_eq!(21, section.page_list.len());
    //    }
}
