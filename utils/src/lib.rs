//mod logging;
pub mod as_any;
pub mod dispatcher;
pub mod fnv1a;
pub mod handles;
pub mod slot_maps;
pub mod split_view;
pub mod squirre13;

pub use rayon;
pub use smol;

pub const IDENTIFIER: &'static str = "Utils";

#[macro_use]
pub mod log;

pub use crate::log::*;
