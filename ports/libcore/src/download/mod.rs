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
    let path = format!("{}/{}.jpg", dir, name);
    println!("{}", &path);
    let mut f = File::create(&path)?;
    f.write_all(&mut buf)?;
    Ok(OutputFile::new("image/jpg", PathBuf::from(&path)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download() {
        let dir = "../../target";
        let name = "baidu";
        let path = format!("{}/{}.jpg", dir, name);
        std::fs::remove_file(path);
        let of = from_url("https://www.baidu.com/img/bd_logo1.png", dir, name).unwrap();
        assert_eq!(true, of.path.exists());
    }
}
