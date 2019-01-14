use crate::errors::Result as FaultTolerance;
use crate::errors::*;
use crate::fetch::http::*;
use std::fs::File;
use std::fs::OpenOptions;
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
    helper.send_get(url);
    from_helper(&mut helper, dir, name)
}

pub fn from_helper(helper: &mut SendHelper, dir: &str, name: &str) -> FaultTolerance<OutputFile> {
    if !helper.done() {
        return Err(err_msg("Please send the request"));
    } else if !helper.succeed()? {
        return Err(err_msg("Download request failed"));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download() {
        let dir = "../../target";
        let name = "mini";
        let path = format!("{}/{}.jpeg", dir, name);
        std::fs::remove_file(path);
        let of = from_url(
            "http://personal.psu.edu/users/w/z/wzz5072/mini.jpg",
            dir,
            name,
        )
        .unwrap();
        assert_eq!(true, of.path.exists());
    }
}
