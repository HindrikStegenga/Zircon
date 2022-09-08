use assets::create_archive_from_directory;
use std::path::*;

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        create_archive_from_directory(
            path.clone().join("assets"),
            path.clone().join("asset_archives").join("assets.zarc"),
            0,
            assets::ArchiveCompressionFormat::ZSTD,
        )
        .await
        .unwrap();
    });
}
