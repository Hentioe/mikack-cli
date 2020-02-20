use crate::CACHE_DIR;
use mikack::error::*;
use mikack::models::Chapter;
use scan_dir::ScanDir;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::PathBuf;
use zip::{write::FileOptions, ZipWriter};

pub trait Exporter {
    fn from_cache(base_dir: &str) -> Result<Self>
    where
        Self: Sized;
    fn expo(&self) -> Result<PathBuf>;
}

pub fn metadata(base_dir: &str) -> Result<Chapter> {
    let mut fpath = PathBuf::from(CACHE_DIR);
    fpath.push(base_dir);
    create_dir_all(&fpath)?;
    fpath.push("metadata.json");
    let mut file = File::open(fpath)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let chapter = serde_json::from_str::<Chapter>(&contents)?;
    Ok(chapter)
}

pub fn archive_dir(dir: &str, dst: &str) -> Result<()> {
    let file = std::fs::File::create(dst).unwrap();
    let mut zip_f = ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);
    archive(dir, &mut Vec::new(), &mut zip_f, &options, "")?;
    zip_f.finish()?;
    Ok(())
}

fn archive(
    dir: &str,
    mut buffer: &mut Vec<u8>,
    mut zip_f: &mut ZipWriter<File>,
    options: &FileOptions,
    parent_dir: &str,
) -> Result<()> {
    ScanDir::all().read(dir, |iter| {
        for (entry, name) in iter {
            if entry.path().is_file() {
                zip_f
                    .start_file(format!("{}{}", parent_dir, name), *options)
                    .unwrap();
                let mut f = File::open(entry.path().to_str().unwrap()).unwrap();
                f.read_to_end(&mut buffer).unwrap();
                zip_f.write_all(&*buffer).unwrap();
                buffer.clear();
            } else {
                let path = &format!("{}{}/", parent_dir, name);
                zip_f
                    .add_directory(path.clone(), FileOptions::default())
                    .unwrap();
                archive(
                    entry.path().to_str().unwrap(),
                    &mut buffer,
                    &mut zip_f,
                    options,
                    path,
                )
                .unwrap();
            }
        }
    })?;
    Ok(())
}

pub mod copy;
pub mod epub;

pub fn gen_expo(format: &str, base_dir: &str) -> Result<Box<dyn Exporter + Send + Sync>> {
    match format {
        "epub" => Ok(Box::new(epub::Epub::from_cache(base_dir)?)),
        "none" => Ok(Box::new(copy::Copy::from_cache(base_dir)?)),
        _ => Err(err_msg(format!("Unsupported format: `{}`", format))),
    }
}
