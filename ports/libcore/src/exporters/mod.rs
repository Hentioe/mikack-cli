pub mod azw3;
pub mod epub;
pub mod mobi;
pub mod pdf;
pub mod prelude;
pub mod zip;

use crate::{checker, errors::*, models::*};

use std::process::{Command, Stdio};

pub fn book_convert(src: &str, dst: &str) -> Result<()> {
    let program = "ebook-convert";
    if !checker::exec_succeed(program, &["--version"]) {
        return Err(err_msg(
            "please install Calibre: https://calibre-ebook.com/download",
        ));
    }

    let status = Command::new(program)
        .arg(&src)
        .arg(&dst)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(err_msg(format!(
            "possible conversion failed with an incorrect exit code: {}",
            status.code().ok_or(err_msg(
                "possible conversion failed and no exit code was obtained"
            ))?
        )))
    }
}
