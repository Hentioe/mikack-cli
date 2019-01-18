use crate::errors::{Result as FaultTolerance, *};
use crate::{
    fetch::{http::*, *},
    get_origin_path,
    progress::*,
};
use reqwest::header::{HeaderValue, REFERER};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub enum Result {
    Ok(OutputFile),
    Err(Error),
}

pub struct OutputFile {
    pub mime: String,
    pub path: PathBuf,
}

impl OutputFile {
    pub fn new(mime: &str, path: PathBuf) -> Self {
        Self {
            mime: mime.to_string(),
            path,
        }
    }
}

pub fn from_url(url: &str, dir: &str, name: &str) -> FaultTolerance<OutputFile> {
    let mut helper = SendHelper::new();
    helper.send_get(url)?;
    Ok(from_helper(&mut helper, dir, name)?)
}

pub fn from_helper(helper: &mut SendHelper, dir: &str, name: &str) -> FaultTolerance<OutputFile> {
    if !helper.done() {
        return Err(err_msg("please send the request"));
    } else if !helper.succeed()? {
        return Err(err_msg("download request failed"));
    }
    let resp = helper.response.as_mut().unwrap();
    let mut buf: Vec<u8> = vec![];
    resp.copy_to(&mut buf)?;
    let path = format!("{}/{}", dir, name);
    let mut f = File::create(&path)?;
    f.write_all(&mut buf)?;
    let mime_s = tree_magic::from_u8(&buf);
    let extension_name = get_extension_from_mime(&mime_s);
    let mut dst_path = PathBuf::from(&path);
    dst_path.set_extension(extension_name);
    std::fs::rename(&path, &dst_path)?;
    Ok(OutputFile::new(&mime_s, PathBuf::from(&dst_path)))
}

fn get_extension_from_mime(mime_s: &str) -> &str {
    let types: Vec<&str> = mime_s.split("/").collect();
    if types.len() < 2 {
        "jpg"
    } else {
        types.get(1).unwrap_or(&"jpg")
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct SectionTrace {
    name: String,
    list: Vec<Page>,
}

impl SectionTrace {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            list: vec![],
        }
    }

    fn add_page(&mut self, page: Page) {
        self.list.push(page);
    }
}

pub fn from_section(section: &mut Section) -> FaultTolerance<Progress> {
    if !section.has_page() {
        return Err(err_msg("does not contain a section list"));
    }
    let dir = get_origin_path(&section.name)?;
    let trace_path = format!("{}/{}", &dir, "trace.json");
    std::fs::create_dir_all(&dir)?;
    let mut trace = SectionTrace::new(&section.name);
    // 如果已存在 trace.json 提取已保存的页信息（图片）
    let mut exists_list: Vec<Page> = vec![];
    if PathBuf::from(&trace_path).exists() {
        let mut json_s = String::new();
        let mut trace_f = File::open(&trace_path)?;
        trace_f.read_to_string(&mut json_s)?;
        match serde_json::from_str::<Value>(&json_s) {
            Ok(trace_v) => {
                for page_v in trace_v["list"]
                    .as_array()
                    .ok_or(err_msg("incorrect trace.json, no 'list' field found"))?
                {
                    exists_list.push(Page {
                        p: page_v["p"]
                            .as_u64()
                            .ok_or(err_msg("incorrect trace.json"))?
                            as u32,
                        url: page_v["url"]
                            .as_str()
                            .ok_or(err_msg("incorrect trace.json"))?
                            .to_owned(),
                        mime: page_v["mime"]
                            .as_str()
                            .ok_or(err_msg("incorrect trace.json"))?
                            .to_owned(),
                        extension: page_v["extension"]
                            .as_str()
                            .ok_or(err_msg("incorrect trace.json"))?
                            .to_owned(),
                    });
                }
            }
            Err(_e) => {
                std::fs::remove_file(&trace_path).unwrap_or(());
            }
        }
    }
    // 开始循环下载
    let progress = Progress::new(section.page_list.len() as u64);
    let mut all_succeed = Ok(());
    for page in &mut section.page_list {
        let exist_p: Vec<&Page> = exists_list.iter().filter(|p| p.url == page.url).collect();
        if exist_p.len() > 0
            && PathBuf::from(format!("{}/{}", &dir, exist_p[0].file_name())).exists()
        {
            let exist = exist_p[0].clone();
            *page = exist.clone();
            trace.add_page(exist.clone());
        } else {
            let mut helper = SendHelper::with_header(REFERER, HeaderValue::from_str(&section.url)?);
            helper.send_get(&page.url)?;
            match from_helper(&mut helper, &dir, &format!("{}", page.p)) {
                Ok(of) => {
                    page.set_mime(&of.mime);
                    page.set_extension((&of).path.extension().unwrap().to_str().unwrap());
                    trace.add_page(page.clone());
                }
                Err(e) => {
                    all_succeed = Err(e);
                    break;
                }
            }
        }
        progress.go();
    }
    // 写入 trace.json
    {
        let mut trace_f = File::create(&trace_path)?;
        trace_f.write_all(serde_json::to_string(&trace)?.as_bytes())?;
    }
    match all_succeed {
        Ok(_) => Ok(progress),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singal_download() {
        let dir = "../../target";
        let name = "mini";
        let path = format!("{}/{}.jpeg", dir, name);
        std::fs::remove_file(&path).unwrap_or(());
        let of = from_url(
            "http://personal.psu.edu/users/w/z/wzz5072/mini.jpg",
            dir,
            name,
        )
        .unwrap();
        assert_eq!(true, of.path.exists());
        std::fs::remove_file(&path).unwrap_or(());
    }

    #[test]
    fn test_from_section_download() {
        let mut section = Section::new(
            "流浪猫的一生  第02话",
            "https://manhua.dmzj.com/liulangmaodeyisheng/81975.shtml#@page=1",
        );
        section.add_page(Page::new(0, "https://images.dmzj.com/l/%E6%B5%81%E6%B5%AA%E7%8C%AB%E7%9A%84%E4%B8%80%E7%94%9F/%E7%AC%AC02%E8%AF%9D/001.jpg"));
        section.add_page(Page::new(1, "https://images.dmzj.com/l/%E6%B5%81%E6%B5%AA%E7%8C%AB%E7%9A%84%E4%B8%80%E7%94%9F/%E7%AC%AC02%E8%AF%9D/003.jpg"));
        section.add_page(Page::new(2, "https://images.dmzj.com/l/%E6%B5%81%E6%B5%AA%E7%8C%AB%E7%9A%84%E4%B8%80%E7%94%9F/%E7%AC%AC02%E8%AF%9D/004.jpg"));
        from_section(&mut section).unwrap();
    }
}
