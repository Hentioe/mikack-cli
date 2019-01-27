use super::{prelude::*, *};
use crate::{get_origin_path, storage};

pub struct Zip {
    pub section: Section,
}

impl Zip {
    pub fn new(section: Section) -> Self {
        Self { section }
    }
}

impl Exporter for Zip {
    fn save(&mut self, output_dir: &str) -> Result<String> {
        // 建立输出目录
        std::fs::create_dir_all(output_dir)?;
        // 下载整个 Section 的资源
        storage::from_section(&mut self.section)?.finish();
        let origin_dir = get_origin_path(&self.section.name)?;
        let dst_file = format!("{}/{}.zip", &output_dir, self.section.fix_slash_name());
        // 使用原始图片目录产生压缩包
        Self::archive_dir(&origin_dir, &dst_file)?;
        Ok(dst_file)
    }
}
