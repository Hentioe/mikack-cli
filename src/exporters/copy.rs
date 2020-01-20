use super::*;
use crate::{CACHE_DIR, OUTPUT_DIR};
use manga_rs::models::Chapter;
use std::fs::{copy, create_dir_all};
use std::path::PathBuf;

pub struct Copy {
    chapter: Chapter,
}

impl Exporter for Copy {
    fn from_cache(base_dir: &str) -> Result<Self> {
        let chapter = metadata(base_dir)?;
        Ok(Self { chapter: chapter })
    }

    fn expo(&self) -> Result<PathBuf> {
        let mut cache_dir = PathBuf::from(CACHE_DIR);
        cache_dir.push(&self.chapter.title);
        let mut output_dir = PathBuf::from(OUTPUT_DIR);
        output_dir.push(&self.chapter.title);
        create_dir_all(&output_dir)?;

        for page in &self.chapter.pages {
            let mut source_file = cache_dir.clone();
            source_file.push(&page.fname);
            let mut target_file = output_dir.clone();
            target_file.push(&page.fname);
            copy(source_file, target_file)?;
        }
        Ok(output_dir)
    }
}
