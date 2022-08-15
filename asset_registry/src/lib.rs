mod archive;
mod descriptors;
mod error;
mod ids;
mod registry;

pub use archive::*;
pub use descriptors::*;
pub use error::*;
pub use ids::*;
pub use registry::*;

#[allow(dead_code)]
pub(crate) const IDENTIFIER: &'static str = "AssetRegistry";
