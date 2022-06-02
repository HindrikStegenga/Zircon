use super::{
    archive::{AssetArchive, AssetArchiveError},
    header::*,
};
use arrayvec::ArrayString;
use tokio::io::AsyncWriteExt;
use xxhash_rust::xxh3::xxh3_64;

#[derive(Debug)]
pub enum ArchiveBuildError {
    Archive(AssetArchiveError),
    IO(tokio::io::Error),
    IdentifierTooLargeError,
}

impl std::error::Error for ArchiveBuildError {}
impl std::fmt::Display for ArchiveBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(e) => e.fmt(f),
            Self::IdentifierTooLargeError => f.write_str("Identifier was too large!"),
            ArchiveBuildError::Archive(e) => e.fmt(f),
        }
    }
}

impl From<AssetArchiveError> for ArchiveBuildError {
    fn from(e: AssetArchiveError) -> Self {
        Self::Archive(e)
    }
}
impl From<tokio::io::Error> for ArchiveBuildError {
    fn from(e: tokio::io::Error) -> Self {
        Self::IO(e)
    }
}

pub struct ArchiveBuilder<'a, F: AsyncWriteExt + Unpin> {
    files: Vec<FileHeader>,
    offset: u64,
    writer: &'a mut F,
}

impl<'a, F: AsyncWriteExt + Unpin> ArchiveBuilder<'a, F> {
    pub async fn new(mut writer: &'a mut F) -> Result<ArchiveBuilder<'a, F>, ArchiveBuildError> {
        AssetArchive::write_magic_value(&mut writer).await?;
        Ok(Self {
            writer,
            files: vec![],
            offset: std::mem::size_of::<u32>() as u64,
        })
    }

    /// Writes a file into the archive.
    pub async fn write_file(
        &mut self,
        identifier: &str,
        format: AssetSerializationFormat,
        blob: &[u8],
        version: u32,
        compression_format: ArchiveCompressionFormat,
    ) -> Result<(), ArchiveBuildError> {
        if identifier.len() > FileHeader::FILE_HEADER_NAME_LEN {
            return Err(ArchiveBuildError::IdentifierTooLargeError);
        }

        let identifier =
            ArrayString::<{ FileHeader::FILE_HEADER_NAME_LEN }>::from(identifier).unwrap();
        let id = xxh3_64(&identifier.as_bytes());

        let mut compressed_size = blob.len();
        let compressed_hash;
        let offset = self.offset;
        // Compress the blob if necessary
        match compression_format {
            ArchiveCompressionFormat::None => {
                // Write blob as is.
                self.writer.write_all(blob).await?;
                self.offset += blob.len() as u64;
                compressed_hash = xxh3_64(&blob);
            }
            ArchiveCompressionFormat::ZSTD => {
                // Compress it first.
                let compressed = zstd::bulk::compress(blob, 0)?;
                compressed_size = compressed.len();
                self.writer.write_all(&compressed).await?;
                self.offset += compressed.len() as u64;
                compressed_hash = xxh3_64(&compressed);
            }
        }

        let header = FileHeader::new(
            identifier,
            id,
            format,
            version,
            offset,
            blob.len() as u64,
            compressed_size as u64,
            compressed_hash,
            compression_format,
        );
        self.files.push(header);
        Ok(())
    }

    /// Writes the header file to the writer and closes up the archive.
    /// On succes returns the borrow writer.
    /// If it fails, the written contents should be considered undefined.
    pub async fn finish(mut self, uuid: uuid::Uuid) -> Result<&'a mut F, ArchiveBuildError> {
        let header = ArchiveHeader::new(uuid, self.files);
        AssetArchive::write_header(header, &mut self.writer).await?;
        Ok(self.writer)
    }
}
