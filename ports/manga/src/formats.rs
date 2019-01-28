use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash)]
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
            let mut formats_by_s: HashMap<&'static str, Format> = HashMap::new();
            let mut formats_by_f: HashMap<Format, &'static str> = HashMap::new();
            $(
                formats_by_s.insert($format.0, $format.1);
                formats_by_f.insert($format.1, $format.0);
            )*
            (formats_by_s, formats_by_f)
        }
    }
}

type FormatBy = (HashMap<&'static str, Format>, HashMap<Format, &'static str>);

lazy_static! {
    static ref FORMATS_BY: FormatBy = append_formats![
        ("epub", Epub),
        ("mobi", Mobi),
        ("azw3", Azw3),
        ("pdf", Pdf),
        ("zip", Zip)
    ];
}

impl ToString for Format {
    fn to_string(&self) -> String {
        FORMATS_BY.1.get(self).unwrap().to_string()
    }
}

impl Format {
    pub fn find(format_s: &str) -> Option<&Format> {
        FORMATS_BY.0.get(format_s.to_lowercase().as_str())
    }
}
