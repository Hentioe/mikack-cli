pub mod archive;
pub mod check;
pub mod errors;
pub mod export;
pub mod fetch;
pub mod jsrt;
pub mod progress;
pub mod storage;

pub const BASE_RES_DIR: &'static str = "manga_res";
pub const CACHE_DIR_NAME: &'static str = ".cache";
pub const ORIGIN_DIR_NAME: &'static str = "origins";
pub const DEFAULT_OUTPUT_DIR: &'static str = "manga_res/outputs";

use std::path::PathBuf;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn get_cache_path(section_name: &str) -> errors::Result<String> {
    let mut path = PathBuf::from(BASE_RES_DIR);
    path.push(section_name);
    path.push(CACHE_DIR_NAME);

    Ok(path
        .to_str()
        .ok_or(errors::err_msg(format!(
            "error getting cache directory for {}",
            section_name
        )))?
        .to_string())
}

pub fn get_origin_path(section_name: &str) -> errors::Result<String> {
    let mut path = PathBuf::from(BASE_RES_DIR);
    path.push(section_name);
    path.push(ORIGIN_DIR_NAME);

    Ok(path
        .to_str()
        .ok_or(errors::err_msg(format!(
            "error getting origin directory for {}",
            section_name
        )))?
        .to_string())
}
