use crate::errors::*;

pub trait Exporter {
    fn save(&mut self) -> Result<String>;
}
