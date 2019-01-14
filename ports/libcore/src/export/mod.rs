pub mod epub;

use crate::errors::*;
use crate::fetch::*;

pub trait Exporter {
    fn save_from_section(&mut self) -> Result<String>;
}
