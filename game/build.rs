use asset_library::archive_directory;
use std::path::*;

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    archive_directory(
        path.clone().join("assets"),
        "assets",
        path.clone().join("asset_archives").join("assets.harchive"),
    )
    .unwrap();
}
