#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod traits;
mod placeholder;
mod json;
mod deserializer;
pub mod functions;
pub mod context;

use traits::*;
use placeholder::*;
use json::*;
pub use functions::*;
pub use deserializer::*;
pub use context::*;