use crate::errors::Result as FaultTolerance;
use crate::errors::*;
use crate::fetch::http::*;
use crate::fetch::*;
use reqwest::header::{HeaderValue, REFERER};
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

pub fn from_section(section: &mut Section) -> FaultTolerance<()> {
    if !section.has_page() {
        return Err(err_msg("does not contain a section list"));
    }
    let dir = format!("manga_res/{}/origins", &section.name);
    std::fs::create_dir_all(&dir)?;
    for page in &mut section.page_list {
        let mut helper = SendHelper::with_header(REFERER, HeaderValue::from_str(&section.url)?);
        helper.send_get(&page.url)?;
        let of = from_helper(&mut helper, &dir, &format!("{}", page.p))?;
        page.set_mime(&of.mime);
        page.set_extension((&of).path.extension().unwrap().to_str().unwrap());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singal_download() {
        let dir = "../../target";
        let name = "mini";
        let path = format!("{}/{}.jpeg", dir, name);
        std::fs::remove_file(&path);
        let of = from_url(
            "http://personal.psu.edu/users/w/z/wzz5072/mini.jpg",
            dir,
            name,
        )
        .unwrap();
        assert_eq!(true, of.path.exists());
        std::fs::remove_file(&path);
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
