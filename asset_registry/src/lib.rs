mod archive;

mod asset_descriptor;
mod asset_source;
mod registry;
#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub(crate) const IDENTIFIER: &'static str = "AssetRegistry";

pub use archive::*;
pub use asset_descriptor::*;
pub use asset_source::*;
pub use registry::*;
