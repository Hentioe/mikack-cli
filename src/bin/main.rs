use indicatif::ProgressBar;
use manga::{exporters::*, *};
use manga_rs::{
    extractors::{self, DomainRoute, Extractor},
    models::*,
};

fn main() -> Result<()> {
    let matches = cli::build_cli().get_matches();
    if let Some(url) = matches.value_of("url") {
        return process_url(url);
    }
    let mut domains = vec![];
    for (i, (domain, name)) in extractors::PLATFORMS.iter().enumerate() {
        domains.push(domain);
        println!("{}. {} ({})", i + 1, name, domain);
    }

    let platform_s = read_input_as_string("\nPlease enter platform number: ")?;
    let domain = domains[platform_s.parse::<usize>()? - 1];
    let extractor =
        extractors::get_extr(domain).expect(&format!("Unsupported platform {}", domain));
    process_index(extractor, 1)?;
    Ok(())
}

fn process_url(url: &str) -> Result<()> {
    let domain_route = extractors::domain_route(url).expect("This link is not supported");
    Ok(match domain_route {
        DomainRoute::Comic(domain) => {
            process_chapters(get_exrt(domain)?, &mut Comic::new("", url))?
        }
        DomainRoute::Chapter(domain) => {
            process_save(get_exrt(domain)?, &mut Chapter::new("", url, 0))?
        }
    })
}

type ExtractorObject = Box<dyn Extractor + Sync + Send>;

fn get_exrt(domain: String) -> Result<&'static ExtractorObject> {
    if let Some(extractor) = extractors::get_extr(&domain) {
        Ok(extractor)
    } else {
        Err(err_msg(format!("Unsupported platform {}", domain)))
    }
}

fn process_index(extractor: &ExtractorObject, index: usize) -> Result<()> {
    let mut comics = extractor.index(index as u32)?;
    for (i, comic) in comics.iter().enumerate() {
        println!("{}. {}", i + 1, comic.title);
    }
    let comic_s = read_input_as_string(&format!(
        "* p{}\nPlease enter comic number (or press Enter to turn pages): ",
        index
    ))?;
    if comic_s.is_empty() {
        return process_index(extractor, index + 1);
    }
    let comic = &mut comics[comic_s.parse::<usize>()? - 1];
    process_chapters(extractor, comic)?;
    Ok(())
}

fn process_chapters(extractor: &ExtractorObject, comic: &mut Comic) -> Result<()> {
    extractor.fetch_chapters(comic)?;
    for (i, chapter) in comic.chapters.iter().enumerate() {
        println!("{}. {}", i + 1, chapter.title);
    }
    let chapter_s = read_input_as_string("\nPlease enter chapter number: ")?;
    let selects = parse_select_rule(&chapter_s)?;
    for n in selects {
        let chapter = &mut comic.chapters[n - 1];
        process_save(extractor, chapter)?;
    }
    Ok(())
}

fn process_save(extractor: &ExtractorObject, chapter: &mut Chapter) -> Result<()> {
    let page_headers = chapter.page_headers.clone();
    let pages_iter = extractor.pages_iter(chapter)?;
    let base_dir = pages_iter.chapter_title_clone();
    let bar = ProgressBar::new(pages_iter.total as u64);
    let mut pages = vec![];
    for mut page in pages_iter {
        let mut resp = get_resp(&page.address, &page_headers)?;
        let mut buf = vec![];
        resp.copy_to(&mut buf)?;
        cache_to(&base_dir, &page.fname, &buf)?;
        if let Some(mime) = resp.headers().get(reqwest::header::CONTENT_TYPE) {
            page.fmime = mime.to_str()?.to_string();
        }
        pages.push(page);
        bar.inc(1);
    }
    chapter.pages = pages;
    let metadata = serde_json::to_string(chapter)?;
    cache_to(&base_dir, "metadata.json", &metadata.as_bytes().to_vec())?;
    let exporter = exporters::epub::Epub::from_cache(&base_dir)?;
    exporter.expo()?;
    Ok(())
}
