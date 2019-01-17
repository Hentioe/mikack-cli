use crate::errors::*;

pub trait Exporter {
    fn save(&mut self, output_dir: &str) -> Result<String>;
}
