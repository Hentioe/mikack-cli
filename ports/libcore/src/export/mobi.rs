use super::{prelude::*, *};

pub struct Mobi {
    pub platform: Platform,
    pub section: Section,
}

impl Mobi {
    pub fn new(platform: Platform, section: Section) -> Self {
        Self { platform, section }
    }
}

impl Exporter for Mobi {
    fn save(&mut self, output_dir: &str) -> Result<String> {
        // 建立输出目录
        std::fs::create_dir_all(output_dir)?;
        let cache_dir = format!("manga_res/{}/.cache", &self.section.name);
        // 缓存 epub
        epub::Epub::new(self.platform.clone(), self.section.clone()).cache()?;
        let cache_file = format!("{}/{}.epub", &cache_dir, &self.section.name);
        let dst_file = format!("{}/{}.mobi", &output_dir, &self.section.name);
        // 转换 epub 缓存到输出目录
        book_convert(&cache_file, &dst_file)?;
        Ok(dst_file)
    }
}
