mod archive;
#[macro_use]
mod asset_descriptor;
mod cache;
mod formats;
mod registry;
#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub(crate) const IDENTIFIER: &'static str = "AssetRegistry";

pub use archive::*;
pub use asset_descriptor::*;
pub use cache::*;
pub use formats::*;
pub use registry::*;
