#![cfg(test)]
#![allow(unused)]
use crate::archive::*;
use crate::FileHeader;
use std::io::Write;
use std::io::{Cursor, Seek, SeekFrom};
use tokio::io::AsyncBufReadExt;
use tokio::test;

#[tokio::test]
async fn test_asynchronous_load() {
    let mut cursor = Cursor::new(Vec::<u8>::with_capacity(1024 * 1024 * 10));
    let mut builder = ArchiveBuilder::new(&mut cursor).await.unwrap();
    let random_data = (0..64)
        .into_iter()
        .map(|_| rand::random())
        .collect::<Vec<u8>>();

    builder
        .write_file(
            "asset.test",
            crate::archive::AssetSerializationFormat::Binary,
            &random_data,
            2334,
            crate::archive::ArchiveCompressionFormat::ZSTD,
        )
        .await
        .unwrap();

    let uuid = uuid::Uuid::new_v4();
    builder.finish(uuid).await.unwrap();
    cursor.seek(SeekFrom::Start(0));
    let archive = AssetArchive::load_from_readable(cursor).await.unwrap();
    let mut buffer = (0..64).into_iter().map(|_| 0).collect::<Vec<u8>>();
    let mut result = archive.read_asset_into(0, &mut buffer).await.unwrap();
    assert_eq!(result, random_data);
}
