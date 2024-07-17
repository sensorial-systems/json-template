#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod traits;
mod placeholder;
mod json;
mod template;
pub mod functions;

use traits::*;
use placeholder::*;
use json::*;
pub use functions::*;
pub use template::*;
