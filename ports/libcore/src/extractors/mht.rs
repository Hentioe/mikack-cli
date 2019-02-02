use super::{prelude::*, *};
use crate::{errors::*, html, http, jsrt, models::*};
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

pub struct Mht;

impl Extractor for Mht {
    fn index(&self, more: u32) -> Result<Vec<Detail>> {
        let url = format!("https://www.manhuatai.com/all_p{}.html", more + 1);
        let mut fll: LinkListConverter<Detail> =
            LinkListConverter::new(&url, "a.sdiv[title]", vec![]);
        fll.set_href_prefix("http://www.manhuatai.com")
            .set_text_in_dom("li.title");
        fll.try_get_list()?.result()
    }

    fn fetch_sections(&self, detail: &mut Detail) -> Result<()> {
        let url = &detail.url;
        let mut fll: LinkListConverter<Section> =
            LinkListConverter::new(&url, "ul[name=\"topiccount\"] > li > a", vec![]);
        fll.set_href_prefix("http://www.manhuatai.com")
            .text_prefix_finder(&|doc| {
                let name =
                    html::find_attr(doc, "meta[property=\"og:title\"]", "content")?.to_string();
                Ok(name)
            });
        let section_list = fll.try_get_list()?.result()?;
        detail.section_list = section_list;
        detail.reverse_section_list();
        Ok(())
    }

    fn fetch_pages(&self, section: &mut Section) -> Result<()> {
        let mut helper = http::SendHelper::new();
        helper.send_get(&section.url)?;

        match helper.result() {
            http::Result::Ok(html_s) => {
                let caps = RE_CODE
                    .captures(&html_s)
                    .ok_or(err_msg(format!("no script code found, {}", &section.url)))?;
                let code = caps
                    .get(1)
                    .ok_or(err_msg(format!(
                        "no encryption code block found, {}",
                        &section.url
                    )))?
                    .as_str();
                let wrapper_code = format!("{}\n{}", code, &DECRYPT_BLOCK);
                // 托管给 JSRT
                let output = jsrt::read_output(&wrapper_code)?;
                let v: Value = serde_json::from_str(&output)?;
                let count = v["count"].as_u64().ok_or(err_msg("no count found"))?;
                let path = v["path"].as_str().ok_or(err_msg("no path found"))?;
                let _start = v["start"].as_u64().ok_or(err_msg("no start found"))?;
                for i in 1..(count + 1) {
                    section.add_page(Page::new(
                        (i - 1) as u32,
                        &format!(
                            "http://mhpic.mh51.com/comic/{}/{}.jpg-mht.middle.webp",
                            &path, i
                        ),
                    ));
                }
                if !section.has_name() {
                    let doc = html::parse_document(&html_s);
                    section.name =
                        html::find_text(&doc, ".mh_readtitle > h1 > strong")?.to_string();
                }
                Ok(())
            }
            http::Result::Err(e) => Err(e),
        }
    }
}

lazy_static! {
    static ref RE_CODE: Regex = Regex::new(r#"var\s*(mh_info\s*=\{[^\}]+\})"#).unwrap();
}

const DECRYPT_BLOCK: &'static str = r#"
window = {
    "\x70\x72\x6f\x6d\x70\x74": function (e) { new Function(e.replace(/./g, function (e) { return String.fromCharCode(e.charCodeAt(0) - 1) }))() }
}
__cr = {
    "\x64\x65\x63\x6f\x64\x65": "ni`jogp/jnhqbui>ni`jogp/jnhqbui/sfqmbdf)0/0h-gvodujpo)b*|sfuvso!Tusjoh/gspnDibsDpef)b/dibsDpefBu)1*.ni`jogp/qbhfje&21*~*"
}
eval(function (e, o, i, r, n, t) {
    if (n = function (e) {
        return (e < 10 ? "" : n(parseInt(e / 10))) + ((e %= 10) > 35 ? String.fromCharCode(e + 29) : e.toString(36))
    }, !"".replace(/^/, String)) {
        for (; i--;) t[n(i)] = r[i] || n(i);
        r = [function (e) {
            return t[e]
        }], n = function () {
            return "\\w+"
        }, i = 1
    }
    for (; i--;) r[i] && (e = e.replace(new RegExp("\\b" + n(i) + "\\b", "g"), r[i]));
    return e
}('4["\\1\\6\\0\\5\\1\\9"](8["\\3\\2\\7\\0\\3\\2"])', 0, 10, "x6f|x70|x65|x64|window|x6d|x72|x63|__cr|x74".split("|"), 0, {})
)

console.log(JSON.stringify({count: mh_info.totalimg, path: mh_info.imgpath, start: mh_info.startimg}))
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mht_index() {
        let list = Mht {}.index(0).unwrap();
        assert_eq!(36, list.len());
    }

    #[test]
    fn test_mht_fetch_sections() {
        let mut detail = Detail::new("斗破苍穹", "https://www.manhuatai.com/doupocangqiong/");
        Mht {}.fetch_sections(&mut detail).unwrap();
        assert_eq!(742, detail.section_list.len());
    }

    #[test]
    fn test_mht_fetch_pages() {
        let mut section = Section::new(
            "斗破苍穹第735话 唐火儿（下）",
            "https://www.manhuatai.com/doupocangqiong/735.html",
        );
        Mht {}.fetch_pages(&mut section).unwrap();
        assert_eq!(8, section.page_list.len());
    }
}
