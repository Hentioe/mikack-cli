use lazy_static::lazy_static;
use std::collections::HashMap;

pub enum Format {
    Epub,
    Mobi,
    Azw3,
    Pdf,
    Zip,
}

use self::Format::*;

lazy_static! {
    static ref EPUB: &'static str = "epub";
    static ref MOBI: &'static str = "mobi";
    static ref AZW3: &'static str = "azw3";
    static ref PDF: &'static str = "pdf";
    static ref ZIP: &'static str = "zip";
}

lazy_static! {
    static ref FORMATS: HashMap<&'static str, Format> = {
        let mut fs = HashMap::new();
        fs.insert(*EPUB, Epub);
        fs.insert(*MOBI, Mobi);
        fs.insert(*AZW3, Azw3);
        fs.insert(*PDF, Pdf);
        fs.insert(*ZIP, Zip);
        fs
    };
}

impl ToString for Format {
    fn to_string(&self) -> String {
        match self {
            Epub => *EPUB,
            Mobi => *MOBI,
            Azw3 => *AZW3,
            Pdf => *PDF,
            Zip => *ZIP,
        }
        .to_owned()
    }
}

impl Format {
    pub fn find(format_s: &str) -> Option<&Format> {
        FORMATS.get(format_s)
    }
}
