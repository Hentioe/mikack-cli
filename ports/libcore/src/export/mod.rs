pub mod epub;
pub mod prelude;

use crate::errors::*;
use crate::fetch::*;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
