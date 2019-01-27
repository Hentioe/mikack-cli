use super::{prelude::*, *};

pub struct Azw3<'a> {
    pub platform: Platform<'a>,
    pub section: Section,
}

impl<'a> Azw3<'a> {
    pub fn new(platform: Platform<'a>, section: Section) -> Self {
        Self { platform, section }
    }
}

impl<'a> Exporter for Azw3<'a> {
    fn save(&mut self, output_dir: &str) -> Result<String> {
        // 建立输出目录
        std::fs::create_dir_all(output_dir)?;
        let cache_dir = format!("manga_res/{}/.cache", &self.section.fix_slash_name());
        // 缓存 epub
        epub::Epub::new(self.platform.clone(), self.section.clone()).cache()?;
        let cache_file = format!("{}/{}.epub", &cache_dir, &self.section.fix_slash_name());
        let dst_file = format!("{}/{}.azw3", &output_dir, &self.section.fix_slash_name());
        // 转换 epub 缓存到输出目录
        book_convert(&cache_file, &dst_file)?;
        Ok(dst_file)
    }
}
