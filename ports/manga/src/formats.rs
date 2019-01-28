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

macro_rules! append_formats {
    ( $( $format:expr ), * ) => {
        {
            let mut formats: HashMap<&'static str, Format> = HashMap::new();
            $(
                formats.insert($format.0, $format.1);
            )*
            formats
        }
    }
}

lazy_static! {
    static ref EPUB: &'static str = "epub";
    static ref MOBI: &'static str = "mobi";
    static ref AZW3: &'static str = "azw3";
    static ref PDF: &'static str = "pdf";
    static ref ZIP: &'static str = "zip";
}

lazy_static! {
    static ref FORMATS: HashMap<&'static str, Format> = append_formats![
        (*EPUB, Epub),
        (*MOBI, Mobi),
        (*AZW3, Azw3),
        (*PDF, Pdf),
        (*ZIP, Zip)
    ];
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
