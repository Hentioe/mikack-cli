use super::{prelude::*, *};
use crate::{errors::*, html, http, jsrt, models::*};
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

pub struct Hhmh;

impl Extractor for Hhmh {
    fn index(&self, more: u32) -> Result<Vec<Detail>> {
        let url = if more > 0 {
            format!("http://www.hhmmoo.com/comic/{}.html", more + 1)
        } else {
            "http://www.hhmmoo.com/comic/".to_string()
        };
        let mut fll: LinkListConverter<Detail> =
            LinkListConverter::new(&url, "#list .cComicList > li > a", vec![]);
        fll.set_href_prefix("http://www.hhmmoo.com");
        fll.try_get_list()?.result()
    }

    fn fetch_sections(&self, detail: &mut Detail) -> Result<()> {
        let url = &detail.url;
        let mut fll: LinkListConverter<Section> =
            LinkListConverter::new(&url, ".cVolUl > li > a", vec![]);
        fll.set_href_prefix("http://www.hhmmoo.com");
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
                // 获取加密需要的 hostname
                let base_url = &section.url.clone();
                let caps = RE_HOSTNAME.captures(base_url).ok_or(err_msg(format!(
                    "did not get the hostname decryption parameter {}",
                    &section.url
                )))?;
                let hostname = caps
                    .get(1)
                    .ok_or(err_msg(format!(
                        "did not get the hostname decryption parameter {}",
                        &section.url
                    )))?
                    .as_str();
                let doc = html::parse_document(&html_s);
                // 获取并设置漫画名称
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
                        .replace(" - 汗汗漫画", "")
                        .to_string();
                }
                // 获取总页数
                let count = doc
                    .select(&html::parse_select("#hdPageCount")?)
                    .next()
                    .ok_or(err_msg(format!(
                        "did not get the total number of pages, {}",
                        &section.url
                    )))?
                    .value()
                    .attr("value")
                    .ok_or(err_msg(format!(
                        "did not get the total number of pages, {}",
                        &section.url
                    )))?
                    .parse::<i32>()?;
                let hd_domain = doc
                    .select(&html::parse_select("#hdDomain")?)
                    .next()
                    .ok_or(err_msg(format!(
                        "did not get the hd_domain decryption parameter {}",
                        &section.url
                    )))?
                    .value()
                    .attr("value")
                    .ok_or(err_msg(format!(
                        "did not get the hd_domain decryption parameter {}",
                        &section.url
                    )))?
                    .split("|")
                    .collect::<Vec<&str>>()
                    .get(0)
                    .ok_or(err_msg(format!(
                        "did not get the hd_domain decryption parameter {}",
                        &section.url
                    )))?
                    .clone();
                for n in (1..(count + 1)).step_by(2) {
                    let url = RE_URL
                        .replace_all(&section.url, format!("/{}.html", n).as_str())
                        .to_string();
                    let mut helper = http::SendHelper::new();
                    helper.send_get(&url)?;
                    match helper.result() {
                        http::Result::Ok(html_s) => {
                            let doc = html::parse_document(&html_s);
                            // 解密当前页图片地址
                            let img_name_attr = doc
                                .select(&html::parse_select("#iBodyQ img")?)
                                .next()
                                .ok_or(err_msg(format!(
                                    "unable to get cipher-text to current image, {}",
                                    &url
                                )))?
                                .value()
                                .attr("name")
                                .ok_or(err_msg(format!(
                                    "unable to get cipher-text to current image, {}",
                                    &url
                                )))?;
                            let vars = format!(
                                "var hostname='{}'; var imgNameAttr='{}';",
                                &hostname, img_name_attr
                            );
                            let wrapper_code = format!("{}\n{}", &vars, &DECRYPT_BLOCK);
                            let cur_url = decryption(&wrapper_code, &hd_domain)?;
                            section.add_page(Page::new((n - 1) as u32, &cur_url));
                            if n < count {
                                // 解密下一页图片地址
                                let img_name_attr = doc
                                    .select(&html::parse_select("#hdNextImg")?)
                                    .next()
                                    .ok_or(err_msg(format!(
                                        "unable to get cipher-text to next image, {}",
                                        &url
                                    )))?
                                    .value()
                                    .attr("value")
                                    .ok_or(err_msg(format!(
                                        "unable to get cipher-text to next image, {}",
                                        &url
                                    )))?;
                                let vars = format!(
                                    "var hostname='{}'; var imgNameAttr='{}';",
                                    &hostname, img_name_attr
                                );
                                let wrapper_code = format!("{}\n{}", &vars, &DECRYPT_BLOCK);
                                let nex_url = decryption(&wrapper_code, &hd_domain)?;
                                section.add_page(Page::new((n) as u32, &nex_url));
                            }
                        }
                        http::Result::Err(e) => return Err(e),
                    }
                }
                Ok(())
            }
            http::Result::Err(e) => Err(e),
        }
    }
}

fn decryption(wrapper_code: &str, hd_domain: &str) -> Result<String> {
    // 托管给 JSRT 并获取结果
    let output = jsrt::read_output(wrapper_code)?;
    let v: Value = serde_json::from_str(&output)?;
    let url = format!(
        "{}{}",
        &hd_domain,
        v["path"].as_str().ok_or(err_msg(format!(
            "decryption failed, full wrapper code: {}",
            &wrapper_code
        )))?
    );
    Ok(url)
}

lazy_static! {
    static ref RE_URL: Regex = Regex::new(r#"/\d+.html"#).unwrap();
    static ref RE_HOSTNAME: Regex = Regex::new(r#"^(https?://[^/]+)/"#).unwrap();
}

const DECRYPT_BLOCK: &'static str = r#"
location = {
    hostname: hostname
}
function unsuan(s) {
    sw = "hhmmoo.com|hhssee.com";
    su = location.hostname.toLowerCase();
    b = false;
    for (i = 0; i < sw.split("|").length; i++) {
        if (su.indexOf(sw.split("|")[i]) > -1) {
            b = true;
            break;
        }
    }
    if (!b) return "";

    x = s.substring(s.length - 1);
    w = "abcdefghijklmnopqrstuvwxyz";
    xi = w.indexOf(x) + 1;
    sk = s.substring(s.length - xi - 12, s.length - xi - 1);
    s = s.substring(0, s.length - xi - 12);
    k = sk.substring(0, sk.length - 1);
    f = sk.substring(sk.length - 1);
    for (i = 0; i < k.length; i++) {
        eval("s=s.replace(/" + k.substring(i, i + 1) + "/g,'" + i + "')");
    }
    ss = s.split(f);
    s = "";
    for (i = 0; i < ss.length; i++) {
        s += String.fromCharCode(ss[i]);
    }
    return s;
}

console.log(JSON.stringify({path: `${unsuan(imgNameAttr)}`}));
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hhmh_index() {
        let list = Hhmh {}.index(0).unwrap();
        assert_eq!(30, list.len());
    }

    #[test]
    fn test_hhmh_fetch_sections() {
        let mut detail = Detail::new("一拳超人", "http://www.hhmmoo.com/manhua15840.html");
        Hhmh {}.fetch_sections(&mut detail).unwrap();
        assert_eq!(321, detail.section_list.len());
    }

    #[test]
    fn test_hhmh_fetch_pages() {
        let mut section = Section::new(
            "妖精的尾巴 第542集",
            "http://www.hhmmoo.com/page285327/1.html?s=4",
        );
        Hhmh {}.fetch_pages(&mut section).unwrap();
        assert_eq!(21, section.page_list.len());
    }
}
