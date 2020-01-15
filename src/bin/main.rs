use indicatif::ProgressBar;
use manga::*;
use manga_rs::extractors;

fn main() -> Result<()> {
    let _matches = cli::build_cli().get_matches();
    let mut domains = vec![];
    for (i, (domain, name)) in extractors::PLATFORMS.iter().enumerate() {
        domains.push(domain);
        println!("{}. {} ({})", i + 1, name, domain);
    }

    let platform_s = read_input_as_string("\nPlease enter platform number: ")?;
    let domain = domains[platform_s.parse::<usize>()? - 1];
    let extractor =
        extractors::get_extr(domain).expect(&format!("Unsupported platform {}", domain));
    let mut comics = extractor.index(1)?;
    for (i, comic) in comics.iter().enumerate() {
        println!("{}. {}", i + 1, comic.title);
    }
    let comic_s = read_input_as_string("\nPlease enter comic number: ")?;
    let comic = &mut comics[comic_s.parse::<usize>()? - 1];
    extractor.fetch_chapters(comic)?;
    for (i, chapter) in comic.chapters.iter().enumerate() {
        println!("{}. {}", i + 1, chapter.title);
    }
    let chapter_s = read_input_as_string("\nPlease enter chapter number: ")?;
    let chapter = &mut comic.chapters[chapter_s.parse::<usize>()? - 1];
    let page_headers = chapter.page_headers.clone();
    let base_dir = chapter.title.clone();
    let pages_iter = extractor.pages_iter(chapter)?;
    let bar = ProgressBar::new(pages_iter.total as u64);
    for page in pages_iter {
        let bytes = get_bytes(&page.address, &page_headers)?;
        save_to(&base_dir, &page.n.to_string(), &bytes)?;
        bar.inc(1);
    }
    Ok(())
}
