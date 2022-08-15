use asset_library::archive_directory;
use asset_registry::create_archive_from_directory;
use std::path::*;

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    archive_directory(
        path.clone().join("assets"),
        "assets",
        path.clone().join("asset_archives").join("assets.harchive"),
        0,
        asset_library::archive::AssetArchiveCompressionFormat::ZSTD,
    )
    .unwrap();
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        create_archive_from_directory(
            path.clone().join("assets"),
            path.clone().join("asset_archives").join("assets.harc"),
            0,
            asset_registry::ArchiveCompressionFormat::ZSTD,
        )
        .await
        .unwrap();
    });
}
