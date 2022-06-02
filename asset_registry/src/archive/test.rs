#![cfg(test)]
#![allow(unused)]
use super::archive::*;
use super::builder::*;
use std::io::{Cursor, Seek, SeekFrom};
use tokio::test;

#[tokio::test]
async fn test_builder() {
    let mut cursor = Cursor::new(Vec::<u8>::with_capacity(1024 * 1024 * 10));
    let mut builder = ArchiveBuilder::new(&mut cursor).await.unwrap();

    let random_data = (0..1_048_576)
        .into_iter()
        .map(|_| rand::random())
        .collect::<Vec<u8>>();

    builder
        .write_file(
            "asset.test",
            super::header::AssetSerializationFormat::None,
            &random_data,
            234234,
            super::header::ArchiveCompressionFormat::ZSTD,
        )
        .await
        .unwrap();

    let uuid = uuid::Uuid::new_v4();
    builder.finish(uuid).await.unwrap();

    // Reset the cursor to the start.
    cursor.seek(SeekFrom::Start(0));

    // Read magic value.
    AssetArchive::read_magic_value(&mut cursor)
        .await
        .expect("Magic value failure.");

    let header = AssetArchive::read_header(&mut cursor).await.unwrap();
    assert_eq!(header.uuid(), uuid);
}
