use super::*;
use crate::{CACHE_DIR, OUTPUT_DIR, VERSION};
use chrono::{offset::Utc, DateTime};
use manga_rs::models::Chapter;
use std::fs::{copy, create_dir_all, remove_dir_all};
use std::path::PathBuf;
use tera::{Context, Tera};
use uuid::Uuid;

pub struct Epub {
    uuid: String,
    chapter: Chapter,
}

static REPO_URL: &'static str = "https://github.com/Hentioe/manga-cli";

impl Epub {
    fn render_start_page(&self) -> Result<String> {
        let template = include_str!("../../template/epub/start.xhtml");
        let mut ctx = Context::new();
        ctx.insert("chapter", &self.chapter);
        ctx.insert("repo", REPO_URL);
        Ok(Tera::one_off(&template, &ctx, false)?)
    }

    fn render_page(&self, fname: &str) -> Result<String> {
        let template = include_str!("../../template/epub/p.xhtml");
        let mut ctx = Context::new();
        ctx.insert("name", &self.chapter.title);
        ctx.insert("fname", &fname);
        Ok(Tera::one_off(&template, &ctx, false)?)
    }

    fn render_metadata_opf(&self) -> Result<String> {
        let template = include_str!("../../template/epub/metadata.opf");
        let mut ctx = Context::new();
        ctx.insert("chapter", &self.chapter);
        ctx.insert("uuid", &self.uuid);
        ctx.insert("version", VERSION);
        ctx.insert("date_time", &DateTime::<Utc>::from(Utc::now()).to_rfc3339());

        Ok(Tera::one_off(&template, &ctx, false)?)
    }

    fn render_stylesheet(&self) -> Result<String> {
        Ok(include_str!("../../template/epub/stylesheet.css").to_string())
    }

    fn render_toc_ncx(&self) -> Result<String> {
        let template = include_str!("../../template/epub/toc.ncx");
        let mut ctx = Context::new();
        ctx.insert("chapter", &self.chapter);
        ctx.insert("uuid", &self.uuid);

        Ok(Tera::one_off(&template, &ctx, false)?)
    }

    fn render_container_xml(&self) -> Result<String> {
        Ok(include_str!("../../template/epub/container.xml").to_string())
    }
}

impl Exporter for Epub {
    fn from_cache(base_dir: &str) -> Result<Self> {
        let chapter = metadata(base_dir)?;
        Ok(Self {
            uuid: Uuid::new_v4().to_hyphenated().to_string(),
            chapter,
        })
    }

    fn expo(&self) -> Result<PathBuf> {
        let base_dir = &self.chapter.title;
        // 写入 start.xhtml
        let start_xhtml = &self.render_start_page()?.as_bytes().to_vec();
        write_to(base_dir, "start.xhtml", start_xhtml)?;
        // 写入页面并复制图片
        for page in &self.chapter.pages {
            let mut page_img_path = PathBuf::from(CACHE_DIR);
            page_img_path.push(base_dir);
            page_img_path.push(&page.fname);

            let mut target_img_page = PathBuf::from(OUTPUT_DIR);
            target_img_page.push(base_dir);
            target_img_page.push(&page.fname);

            copy(page_img_path, target_img_page)?;

            let page_xhtml = &self.render_page(&page.fname)?.as_bytes().to_vec();
            write_to(base_dir, &format!("{}.xhtml", page.n), page_xhtml)?;
        }
        // 写入 metadata.opf
        let metadata_opf = &self.render_metadata_opf()?.as_bytes().to_vec();
        write_to(base_dir, "metadata.opf", metadata_opf)?;
        // 写入 mimetype
        let mimetype = &"application/epub+zip".as_bytes().to_vec();
        write_to(base_dir, "mimetype", mimetype)?;
        // 写入 stylesheet.css
        let stylesheet_css = &self.render_stylesheet()?.as_bytes().to_vec();
        write_to(base_dir, "stylesheet.css", stylesheet_css)?;
        // 写入 toc.ncx
        let toc_ncx = &self.render_toc_ncx()?.as_bytes().to_vec();
        write_to(base_dir, "toc.ncx", toc_ncx)?;
        // 写入 META-INF/container.xml
        let container_xml = &self.render_container_xml()?.as_bytes().to_vec();
        let mut meta_inf = PathBuf::from(OUTPUT_DIR);
        meta_inf.push(base_dir);
        meta_inf.push("META-INF");
        create_dir_all(&meta_inf)?;
        write_to(base_dir, "META-INF/container.xml", container_xml)?;

        let mut epub_dir = PathBuf::from(OUTPUT_DIR);
        epub_dir.push(base_dir);
        let mut epub_file = PathBuf::from(OUTPUT_DIR);
        epub_file.push(format!("{}.epub", self.chapter.title));
        archive_dir(epub_dir.to_str().unwrap(), epub_file.to_str().unwrap())?;
        remove_dir_all(&epub_dir)?;
        Ok(epub_file)
    }
}

fn write_to(base_dir: &str, name: &str, bytes: &Vec<u8>) -> Result<()> {
    let mut fpath = PathBuf::from(OUTPUT_DIR);
    fpath.push(base_dir);
    create_dir_all(&fpath)?;
    fpath.push(name);
    let mut file = File::create(fpath)?;
    Ok(file.write_all(bytes)?)
}
