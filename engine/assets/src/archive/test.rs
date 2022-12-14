#![cfg(test)]
#![allow(unused)]
use super::archive::*;
use super::builder::*;
use crate::FileHeader;
use std::io::Write;
use std::io::{Cursor, Seek, SeekFrom};
use tokio::io::AsyncBufReadExt;
use tokio::test;

#[tokio::test]
async fn test_builder() {
    let mut cursor = Cursor::new(Vec::<u8>::with_capacity(1024 * 1024 * 10));
    let mut builder = ArchiveBuilder::new(&mut cursor).await.unwrap();
    let random_data = (0..64)
        .into_iter()
        .map(|_| rand::random())
        .collect::<Vec<u8>>();

    builder
        .write_file(
            "asset.test",
            crate::AssetSerializationFormat::Binary,
            &random_data,
            2334,
            crate::ArchiveCompressionFormat::ZSTD,
        )
        .await
        .unwrap();

    let uuid = uuid::Uuid::new_v4();
    builder.finish(uuid).await.unwrap();

    // Reset the cursor to the start.
    cursor.seek(SeekFrom::Start(0));

    // Read magic value.
    read_magic_value(&mut cursor)
        .await
        .expect("Magic value failure.");

    // Read header
    let header = read_header(&mut cursor).await.unwrap();
    assert_eq!(header.uuid(), uuid);
    // Read the file into the buffer.
    let mut buffer = vec![0; header.files().first().unwrap().byte_count() as usize];
    read_file_into_buffer(header.files().first().unwrap(), &mut cursor, &mut buffer)
        .await
        .unwrap();
    assert_eq!(random_data, buffer);
}
