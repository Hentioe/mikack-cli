use manga_bot::errors::*;
use scan_dir::ScanDir;
use std::path::PathBuf;

pub fn delete_cache() -> Result<()> {
    let mut failed_count = 0;
    let cache_dir_list = find_all_cache_dir()?;
    for cache_dir in &cache_dir_list {
        if std::fs::remove_dir_all(cache_dir).is_err() {
            failed_count = failed_count + 1;
        }
    }
    println!("");
    println!(
        "{} cache directories deleted",
        (cache_dir_list.len() - failed_count),
    );
    Ok(())
}

fn find_all_cache_dir() -> Result<Vec<String>> {
    let mut cache_dir_list: Vec<PathBuf> = vec![];
    ScanDir::dirs().read(manga_bot::BASE_RES_DIR, |iter| {
        for (entry, name) in iter {
            if "outputs" != name {
                let mut path = entry.path();
                path.push(manga_bot::CACHE_DIR_NAME);
                cache_dir_list.push(path);
            }
        }
    })?;
    Ok(cache_dir_list
        .iter()
        .filter(|p| p.exists())
        .map(|p| p.clone())
        .filter(|p| p.to_str().is_some())
        .map(|p| p.to_str().unwrap().to_owned())
        .collect())
}
