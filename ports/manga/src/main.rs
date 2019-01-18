use console::{style, Emoji};
use lazy_static::lazy_static;
use libcore::{
    errors::*,
    export::{prelude::*, *},
    fetch::{prelude::*, *},
};
use regex::Regex;
use std::io::prelude::*;

mod clean;
mod cli;

static LOOKING_GLASS: Emoji = Emoji("🔍  ", "");
static TRUCK: Emoji = Emoji("🚚  ", "");

lazy_static! {
    static ref DMZJ: Platform = Platform::new("动漫之家", "https://manhua.dmzj.com");
    static ref HHMH: Platform = Platform::new("汗汗漫画", "http://www.hhmmoo.com");
    static ref RE_DETAIL_DMZJ: Regex =
        Regex::new(r#"https?://manhua\.dmzj\.com/[^/]+/\d+\.shtml"#).unwrap();
    static ref RE_DETAIL_HHMH: Regex =
        Regex::new(r#"https?://www\.hhmmoo\.com/page\d+/\d+\.html"#).unwrap();
    static ref RE_SECTION_DMZJ: Regex =
        Regex::new(r#"^https?://manhua\.dmzj\.com/[^/]+/$"#).unwrap();
    static ref RE_SECTION_HHMH: Regex =
        Regex::new(r#"^https?://www\.hhmmoo\.com/manhua\d+\.html$"#).unwrap();
}

fn main() -> Result<()> {
    env_logger::init();
    let matches = cli::build_cli().get_matches();
    let output_dir = matches
        .value_of("output-directory")
        .unwrap_or(libcore::DEFAULT_OUTPUT_DIR);
    if matches.is_present("clean") {
        clean::delete_cache()?;
    } else if matches.value_of("url").is_some() {
        let url = matches.value_of("url").unwrap();
        analysis_url(url, output_dir)?;
    } else {
        println!(
            "Welcome to manga ({})! There are huge manga resources available for direct save.",
            &VERSION
        );
        println!("Yes, any ideas or problems can be discussed at https://github.com/Hentioe/manga-rs/issues.");
        from_source_list(output_dir)?;
    }
    Ok(())
}

fn from_source_list(output_dir: &str) -> Result<()> {
    println!("They are our source of resources:");
    let source_list = gen_sources();
    for (i, (p, _f)) in source_list.iter().enumerate() {
        println!("{}. {}", i + 1, p.name)
    }
    println!("==> Please choose a platform (e.g: 1, want to support your favorite platform? Go to the issue and tell me!)");
    print!("==> ");
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let n = input.trim().parse::<u32>()?;
    let (_, fetcher): &(Platform, Box<&Fetcher>) = source_list
        .get((n - 1) as usize)
        .ok_or(err_msg("no platform selected"))?;
    let mut more = 0;
    loop {
        let detail_list = fetcher.index(more)?;
        for (i, detail) in detail_list.iter().enumerate() {
            println!("{}. {}", i + 1, &detail.name);
        }
        println!("==> Please choose a detail (e.g: 1, Enter to go to the next list)");
        print!("==> ");
        std::io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        input = input.trim().to_string();
        if input == "" {
            more = more + 1;
            continue;
        } else {
            let n = input.parse::<u32>()?;
            let detail = detail_list
                .get((n - 1) as usize)
                .ok_or(err_msg("no detail selected"))?;
            analysis_url(&detail.url, output_dir)?;
            break;
        }
    }
    Ok(())
}

fn analysis_url(url: &str, output_dir: &str) -> Result<()> {
    let section_matches: [(&Regex, &Fetcher, Platform); 2] = [
        (
            &RE_DETAIL_DMZJ,
            &upstream::Dmzj {} as &Fetcher,
            DMZJ.clone(),
        ),
        (
            &RE_DETAIL_HHMH,
            &upstream::Hhmh {} as &Fetcher,
            HHMH.clone(),
        ),
    ];
    let mut passed = false;
    for (re, fr, p) in section_matches.iter() {
        if re.find(&url).is_none() {
            continue;
        } else {
            save(&url, *fr, p.clone(), output_dir)?;
            passed = true;
            break;
        }
    }
    if !passed {
        // 作为 Detail url 继续检测
        let detail_matches: [(&Regex, &Fetcher, Platform); 2] = [
            (
                &RE_SECTION_DMZJ,
                &upstream::Dmzj {} as &Fetcher,
                DMZJ.clone(),
            ),
            (
                &RE_SECTION_HHMH,
                &upstream::Hhmh {} as &Fetcher,
                HHMH.clone(),
            ),
        ];

        for (re, fr, p) in detail_matches.iter() {
            if re.find(&url).is_none() {
                continue;
            } else {
                let mut detail = Detail::new(UNKNOWN_NAME, &url);

                println!(
                    "{} {}Searching list...",
                    style("[1/3]").bold().dim(),
                    LOOKING_GLASS
                );
                fr.fetch_sections(&mut detail)?;
                println!("[1/3] {} Done!", Emoji("✨", ":-)"));
                println!("{} {}Selecting list...", style("[3/2]").bold().dim(), TRUCK);
                for (i, sec) in detail.section_list.iter().enumerate() {
                    println!("{}", format!("{}. {}", (i + 1), &sec.name));
                }
                print!("==> Select to save (eg: 1,2,3, 4-6, ^5)\n==> ");
                let mut input = String::new();
                std::io::stdout().flush()?;
                std::io::stdin().read_line(&mut input)?;
                let select_list = parse_section_list(&input.trim());
                println!("[3/2] {} Done!", Emoji("✨", ":-)"));
                println!(
                    "{} {}Queue processing...",
                    style("[3/3]").bold().dim(),
                    TRUCK
                );
                println!(
                    "[3/3] ------ [{}] ------",
                    format!("{}/{}", 0, select_list.len())
                );
                let mut failed_count = 0;
                for (cur, s) in select_list.iter().enumerate() {
                    if let Some(sec) = detail.section_list.get(*s as usize) {
                        if save(&sec.url, *fr, p.clone(), output_dir).is_err() {
                            failed_count = failed_count + 1;
                        }
                    }
                    println!(
                        "[3/3] ------ [{}] ------",
                        format!("{}/{}", cur + 1, select_list.len())
                    );
                }
                println!("[3/3] {} Done!", Emoji("✨", ":-)"));
                println!(
                    "Result: {} saved; {} failed",
                    (select_list.len() - failed_count),
                    failed_count
                );
                passed = true;
                break;
            }
        }
    }
    if !passed {
        return Err(err_msg("invalid or unsupported url"));
    }
    Ok(())
}

fn save(url: &str, fetcher: &Fetcher, platform: Platform, output_dir: &str) -> Result<String> {
    let mut section = Section::new(UNKNOWN_NAME, url);

    println!(
        "{} {}Fetching pages...",
        style("[1/2]").bold().dim(),
        LOOKING_GLASS
    );
    fetcher.fetch_pages(&mut section)?;
    println!("[1/2] {} Done!", Emoji("✨", ":-)"));
    println!("{} {}Saving epub...", style("[2/2]").bold().dim(), TRUCK);
    let path = epub::Epub::new(platform, section).save(output_dir)?;
    println!("[2/2] {} Done!", Emoji("✨", ":-)"));
    println!("Succeed: {}", &path);
    Ok(path)
}

fn parse_section_list(input_s: &str) -> Vec<i32> {
    let re = regex::Regex::new("(,|，)").unwrap();
    let multi_t: Vec<&str> = re.split(&input_s).map(|s| s.trim()).collect();
    // 剥离 ^n
    let excludes: Vec<i32> = multi_t
        .iter()
        .filter(|s| s.starts_with("^"))
        .map(|s| {
            if let Ok(i) = s[1..s.len()].parse::<i32>() {
                i
            } else {
                -1
            }
        })
        .collect();
    // 剥离 n
    let mut ones: Vec<i32> = multi_t
        .iter()
        .filter(|s| s.parse::<i32>().is_ok())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    // 将 s-e 范围数字展开并添加至 ones 中
    for range in multi_t.iter().filter(|s| s.find("-").is_some()) {
        let (start, end) = {
            let rs = range.split("-").collect::<Vec<&str>>();
            (rs[0], rs[1])
        };
        if let Ok(s) = start.parse::<i32>() {
            if let Ok(e) = end.parse::<i32>() {
                if s < e {
                    for n in s..(e + 1) {
                        ones.push(n);
                    }
                }
            }
        }
    }
    ones.iter()
        .filter(|i| !excludes.contains(i))
        .map(|i| *i - 1)
        .collect()
}

fn gen_sources() -> Vec<(Platform, Box<&'static Fetcher>)> {
    vec![
        (DMZJ.clone(), Box::new(&upstream::Dmzj {} as &Fetcher)),
        (HHMH.clone(), Box::new(&upstream::Hhmh {} as &Fetcher)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_section_list() {
        let input = "1, 2, 3, 4-8， ^5, ^2";
        let result = parse_section_list(&input);
        for i in [0, 2, 3, 5, 6, 7].iter() {
            assert!(result.contains(i));
        }
    }
}
