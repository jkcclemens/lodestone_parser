#![feature(crate_visibility_modifier)]

#[cfg(feature = "logic")] extern crate failure;
#[cfg(feature = "logic")] #[macro_use] extern crate lazy_static;
pub extern crate ffxiv_types;
#[macro_use] extern crate serde_derive;

#[cfg(feature = "logic")]
pub mod error;
#[cfg(feature = "logic")]
pub mod logic;

pub mod models;
crate mod util;

#[cfg(feature = "logic")]
pub use crate::logic::*;
