use super::{epub::*, zip::*};
use crate::{errors::*, fix_slash, storage};
use scan_dir::ScanDir;
use std::io::prelude::*;
use std::{fs::File, path::PathBuf};
use zip::{write::FileOptions, ZipWriter};

pub trait Exporter {
    fn save(&mut self, output_dir: &str) -> Result<String>;
}

impl<'a> Epub<'a> {
    pub fn cache(&mut self) -> Result<()> {
        let cache_dir = format!("manga_res/{}/.cache", &self.section.fix_slash_name());
        let cache_file = format!("{}/{}.epub", &cache_dir, &self.section.fix_slash_name());
        if PathBuf::from(&cache_file).exists() {
            return Ok(());
        }
        // 下载整个 Section 的资源
        storage::from_section(&mut self.section)?.finish();
        // 建立缓存目录
        let cache_epub_dir = format!("{}/epub", &cache_dir);
        std::fs::create_dir_all(&cache_dir)?;
        std::fs::create_dir_all(&cache_epub_dir)?;
        let meta_dir = format!("{}/META-INF", &cache_epub_dir);
        std::fs::create_dir_all(&meta_dir)?;
        // 注入变量并输出 EPUB 结构
        // start.xhtml
        let mut start_xhtml = File::create(format!("{}/start.xhtml", &cache_epub_dir))?;
        start_xhtml.write_all(self.render_start_xhtml().as_bytes())?;
        // 循环写入所有的图片页面和文件
        for page in &self.section.page_list {
            let img_name = format!("{}.{}", &page.p, &page.extension);
            let mut img_xhtml = File::create(format!("{}/{}.html", &cache_epub_dir, page.p))?;
            {
                img_xhtml.write_all(
                    self.render_page_html(&page.p.to_string(), &img_name)
                        .as_bytes(),
                )?;
            }
            let origin_path = format!(
                "{}/{}/origins/{}",
                "manga_res",
                &self.section.fix_slash_name(),
                &img_name
            );
            std::fs::copy(&origin_path, format!("{}/{}", &cache_epub_dir, &img_name))?;
            // 复制第一张图为封面
            if page.p == 0 {
                std::fs::copy(
                    &origin_path,
                    format!(
                        "{}/{}",
                        &cache_epub_dir,
                        format!("cover.{}", &page.extension)
                    ),
                )?;
            }
        }
        // 写入 metadata.opf
        let mut metadata = File::create(format!("{}/metadata.opf", &cache_epub_dir))?;
        metadata.write_all(self.render_metadata_opf().as_bytes())?;
        // 写入 mimetype
        let mut mimetype = File::create(format!("{}/mimetype", &cache_epub_dir))?;
        mimetype.write_all("application/epub+zip".as_bytes())?;
        // 写入 stylesheet.css
        let mut stylesheet = File::create(format!("{}/stylesheet.css", &cache_epub_dir))?;
        stylesheet.write_all(self.render_stylesheet().as_bytes())?;
        // 写入 toc.ncx
        let mut toc = File::create(format!("{}/toc.ncx", &cache_epub_dir))?;
        toc.write_all(self.render_toc_ncx().as_bytes())?;
        // 写入 META-INF/container.xml
        let mut container = File::create(format!("{}/container.xml", &meta_dir))?;
        container.write_all(self.render_container_xml().as_bytes())?;

        // 打包成 epub
        Zip::archive_dir(&cache_epub_dir, &cache_file)?;
        Ok(())
    }
}

impl Zip {
    pub fn archive_dir(dir: &str, dst: &str) -> Result<()> {
        let file = std::fs::File::create(dst).unwrap();
        let mut zip_f = ZipWriter::new(file);
        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);
        Self::archive(dir, &mut Vec::new(), &mut zip_f, &options, "")?;
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
                    let path = &format!("{}/", fix_slash!(format!("{}{}", parent_dir, name)));
                    zip_f
                        .add_directory(path.clone(), FileOptions::default())
                        .unwrap();
                    Self::archive(
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
}
