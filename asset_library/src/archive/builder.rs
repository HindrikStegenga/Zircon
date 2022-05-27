use super::*;
use std::io::{BufWriter, Seek, SeekFrom, Write};

#[derive(Debug)]
pub struct AssetArchiveMountPointBuilder {
    mount_point: String,
    version: u64,
    archive_builder: AssetArchiveBuilder,
    written_files: Vec<AssetArchiveFileHeader>,
}

impl AssetArchiveMountPointBuilder {
    fn new(
        archive_builder: AssetArchiveBuilder,
        mount_point: impl AsRef<str>,
        version: u64,
    ) -> Self {
        Self {
            archive_builder,
            mount_point: mount_point.as_ref().to_lowercase(),
            version,
            written_files: Vec::with_capacity(16),
        }
    }

    pub fn write_file(
        mut self,
        identifier: impl AsRef<str>,
        format: impl AsRef<str>,
        uncompressed_blob: &[u8],
        compression_format: AssetArchiveCompressionFormat,
    ) -> Result<Self, (Self, AssetArchiveError)> {
        use AssetArchiveCompressionFormat::{None, LZ4};
        match compression_format {
            None => {
                match self.archive_builder.writer.write(uncompressed_blob) {
                    Ok(v) => v,
                    Err(e) => return Err((self, AssetArchiveError::Io(e))),
                };
                self.written_files.push(AssetArchiveFileHeader::new(
                    identifier.as_ref().to_lowercase(),
                    format.as_ref().to_lowercase(),
                    self.archive_builder.offset,
                    uncompressed_blob.len() as u64,
                    uncompressed_blob.len() as u64,
                    None,
                ));
                self.archive_builder.offset += uncompressed_blob.len() as u64;
                Ok(self)
            }
            LZ4 => {
                let compressed = lz4_flex::compress(uncompressed_blob);
                match self.archive_builder.writer.write(&compressed) {
                    Ok(v) => v,
                    Err(e) => return Err((self, AssetArchiveError::Io(e))),
                };
                self.written_files.push(AssetArchiveFileHeader::new(
                    identifier.as_ref().to_lowercase(),
                    format.as_ref().to_lowercase(),
                    self.archive_builder.offset,
                    compressed.len() as u64,
                    uncompressed_blob.len() as u64,
                    LZ4,
                ));
                self.archive_builder.offset += compressed.len() as u64;
                Ok(self)
            }
            AssetArchiveCompressionFormat::ZSTD => {
                let compressed = match zstd::bulk::compress(uncompressed_blob, 0) {
                    Ok(v) => v,
                    Err(e) => return Err((self, AssetArchiveError::Io(e))),
                };
                match self.archive_builder.writer.write(&compressed) {
                    Ok(v) => v,
                    Err(e) => return Err((self, AssetArchiveError::Io(e))),
                };
                self.written_files.push(AssetArchiveFileHeader::new(
                    identifier.as_ref().to_lowercase(),
                    format.as_ref().to_lowercase(),
                    self.archive_builder.offset,
                    compressed.len() as u64,
                    uncompressed_blob.len() as u64,
                    AssetArchiveCompressionFormat::ZSTD,
                ));
                self.archive_builder.offset += compressed.len() as u64;
                Ok(self)
            }
        }
    }

    pub fn finish(mut self) -> AssetArchiveBuilder {
        self.archive_builder
            .written_mounts
            .push(AssetArchiveMountPointHeader::new(
                self.version,
                self.mount_point,
                self.written_files,
            ));
        self.archive_builder
    }
}

#[derive(Debug)]
pub struct AssetArchiveBuilder {
    writer: BufWriter<File>,
    written_mounts: Vec<AssetArchiveMountPointHeader>,
    offset: u64,
}

impl AssetArchiveBuilder {
    pub fn new(mut file: File) -> Result<Self, AssetArchiveError> {
        file.seek(SeekFrom::Start(0))?;
        Ok(Self {
            writer: BufWriter::new(file),
            written_mounts: Vec::with_capacity(16),
            offset: 0,
        })
    }

    pub fn add_mount_point(
        self,
        mount_point: impl AsRef<str>,
        version: u64,
    ) -> Result<AssetArchiveMountPointBuilder, (Self, AssetArchiveError)> {
        match self
            .written_mounts
            .iter()
            .find(|e| e.mount_point() == mount_point.as_ref().to_lowercase())
        {
            Some(_) => return Err((self, AssetArchiveError::InvalidMountPoint)),
            None => (),
        }

        Ok(AssetArchiveMountPointBuilder::new(
            self,
            mount_point,
            version,
        ))
    }

    pub fn finish(self) -> Result<(), AssetArchiveError> {
        let header = AssetArchiveHeader::new(self.written_mounts);
        let mut writer = self.writer;
        let cbor_header = serde_cbor::to_vec(&header)?;
        let compressed_header = zstd::bulk::compress(&cbor_header, 0)?;
        writer.write(&compressed_header)?;
        let uncompressed_size_bytes = u64::to_le_bytes(cbor_header.len() as u64);
        writer.write(&uncompressed_size_bytes)?;
        let compressed_size_bytes = u64::to_le_bytes(compressed_header.len() as u64);
        writer.write(&compressed_size_bytes)?;
        writer.flush()?;
        Ok(())
    }
}
