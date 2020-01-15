pub use manga_rs::error::*;
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{stdin, stdout, Write};

pub mod cli;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn read_input_as_string(msg: &str) -> Result<String> {
    let mut s = String::new();
    print!("{}", msg);
    stdout().flush()?;
    stdin().read_line(&mut s)?;
    Ok(s.trim().to_string())
}

pub fn save_to(base_dir: &str, name: &str, bytes: &Vec<u8>) -> Result<()> {
    fs::create_dir_all(format!("_output/{}", base_dir))?;
    let mut file = File::create(format!("_output/{}/{}.jpg", base_dir, name))?;
    file.write_all(bytes)?;
    Ok(())
}

pub fn get_bytes(url: &str, headers: &HashMap<String, String>) -> Result<Vec<u8>> {
    let mut header_map = HeaderMap::new();
    for (key, value) in headers {
        header_map.insert(
            HeaderName::from_bytes(key.as_bytes())?,
            HeaderValue::from_str(&value)?,
        );
    }
    let client = Client::new().get(url).headers(header_map);
    let mut resp = client.send()?;
    let mut buf: Vec<u8> = vec![];
    resp.copy_to(&mut buf)?;
    Ok(buf)
}
