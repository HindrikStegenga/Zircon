use crate::asset_cache::AssetCache;
use crate::{ArchiveBuilder, AssetArchive, AssetRegistry};
use crate::{ArchiveCompressionFormat, AssetSerializationFormat};
use std::io::Cursor;
use std::num::NonZeroUsize;
use std::sync::Arc;
use utils::dispatcher::Dispatcher;

const KB: usize = 1024;
const MB: usize = 1024 * 1024;
const GB: usize = 1024 * 1024 * 1024;

#[test]
fn test_asset_cache() {
    let dispatcher = Arc::new(
        Dispatcher::new(
            Some(unsafe { NonZeroUsize::new_unchecked(1) }),
            unsafe { NonZeroUsize::new_unchecked(1) },
            Some(unsafe { NonZeroUsize::new_unchecked(1) }),
            unsafe { NonZeroUsize::new_unchecked(1) },
        )
        .unwrap(),
    );
    let base_dispatcher = Arc::clone(&dispatcher);
    let manager = base_dispatcher.spawn_async_blocking(async {
        let mut cursor = Cursor::new(Vec::<u8>::with_capacity(8 * MB));
        let mut builder = ArchiveBuilder::new(&mut cursor).await.unwrap();
        let blob = (0..2 * MB)
            .into_iter()
            .map(|_| rand::random())
            .collect::<Vec<u8>>();
        builder
            .write_file(
                "test",
                AssetSerializationFormat::Binary,
                &blob,
                0,
                ArchiveCompressionFormat::ZSTD,
            )
            .await
            .unwrap();
        let uuid = uuid::Uuid::new_v4();
        builder.finish(uuid).await.unwrap();
        let archive = AssetArchive::load_from_readable(cursor).await.unwrap();

        let mut registry = AssetRegistry::<Cursor<Vec<u8>>>::default();
        registry.register_asset_archive(archive).unwrap();
        let registry = Arc::new(registry);

        AssetCache::<Cursor<Vec<u8>>>::new(Arc::clone(&registry), dispatcher)
    });

    // manager
    //     .request_binary(asset_id!(test))
    //     .expect("Could not request binary.");
    let handle = manager
        .request_binary_synchronous(asset_id!(test))
        .expect("Could not request binary synchronously");
    assert!(handle.read().is_some());
}
