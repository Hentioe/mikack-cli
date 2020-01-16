use lazy_static::lazy_static;
pub use manga_rs::error::*;
use regex::Regex;
use reqwest::{
    blocking::{Client, Response},
    header::{HeaderMap, HeaderName, HeaderValue},
};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;

pub mod cli;
pub mod exporters;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

lazy_static! {
    static ref SELECT_SEPARATOR_RE: Regex = Regex::new("(,|，)").unwrap();
}

pub fn read_input_as_string(msg: &str) -> Result<String> {
    let mut s = String::new();
    print!("{}", msg);
    stdout().flush()?;
    stdin().read_line(&mut s)?;
    Ok(s.trim().to_string())
}

pub static OUTPUT_DIR: &'static str = "_output";
pub static CACHE_DIR: &'static str = "_cache";

pub fn cache_to(base_dir: &str, name: &str, bytes: &Vec<u8>) -> Result<()> {
    let mut base_path = PathBuf::from(CACHE_DIR);
    base_path.push(base_dir);
    save_to(base_path, name, bytes)
}

pub fn save_to(base_path: PathBuf, name: &str, bytes: &Vec<u8>) -> Result<()> {
    fs::create_dir_all(&base_path)?;
    let mut fpath = base_path.clone();
    fpath.push(name);
    let mut file = File::create(fpath)?;
    file.write_all(bytes)?;
    Ok(())
}

pub fn get_resp(url: &str, headers: &HashMap<String, String>) -> Result<Response> {
    let mut header_map = HeaderMap::new();
    for (key, value) in headers {
        header_map.insert(
            HeaderName::from_bytes(key.as_bytes())?,
            HeaderValue::from_str(&value)?,
        );
    }
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?
        .get(url)
        .headers(header_map);
    Ok(client.send()?)
}

pub fn parse_select_rule(input_s: &str) -> Result<Vec<usize>> {
    let multi_t: Vec<&str> = SELECT_SEPARATOR_RE
        .split(&input_s)
        .map(|s| s.trim())
        .collect();

    // 剥离 ^n
    let excludes: Vec<i32> = multi_t
        .iter()
        .filter(|s| s.starts_with("^"))
        .map(|s| s[1..s.len()].parse::<i32>().unwrap_or(-1))
        .collect();
    // 剥离 n
    let mut ones: Vec<i32> = multi_t
        .iter()
        .filter(|s| s.parse::<usize>().is_ok())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    // 将 s-e 范围数字展开并添加至 ones 中
    for range in multi_t.iter().filter(|s| s.find("-").is_some()) {
        let (start, end) = {
            let rs = range.split("-").collect::<Vec<&str>>();
            (rs[0].parse::<usize>()?, rs[1].parse::<usize>()?)
        };
        if start < end {
            for n in start..(end + 1) {
                ones.push(n as i32);
            }
        }
    }
    Ok(ones
        .iter()
        .filter(|i| !excludes.contains(i))
        .map(|i| *i as usize)
        .collect())
}
