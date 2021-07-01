use super::*;
use std::io::{BufWriter, Seek, SeekFrom, Write};

pub struct AssetArchiveBuilder {
    writer: BufWriter<File>,
    written_blobs: Vec<AssetArchiveFileHeader>,
    offset: u64,
}

impl AssetArchiveBuilder {
    pub fn new(mut file: File) -> Result<Self, AssetArchiveError> {
        file.seek(SeekFrom::Start(0))?;
        Ok(Self {
            writer: BufWriter::new(file),
            written_blobs: Vec::with_capacity(16),
            offset: 0,
        })
    }

    pub fn write_blob(
        mut self,
        uncompressed_blob: &[u8],
        compression_format: AssetArchiveCompressionFormat,
    ) -> Result<Self, AssetArchiveError> {
        use AssetArchiveCompressionFormat::{None, LZ4};
        match compression_format {
            None => {
                self.writer.write(uncompressed_blob)?;
                self.written_blobs.push(AssetArchiveFileHeader::new(
                    self.offset,
                    uncompressed_blob.len() as u64,
                    uncompressed_blob.len() as u64,
                    None,
                ));
                self.offset += uncompressed_blob.len() as u64;
                Ok(self)
            }
            LZ4 => {
                let compressed = lz4_flex::compress(uncompressed_blob);
                self.writer.write(&compressed)?;
                self.written_blobs.push(AssetArchiveFileHeader::new(
                    self.offset,
                    compressed.len() as u64,
                    uncompressed_blob.len() as u64,
                    LZ4,
                ));
                self.offset += compressed.len() as u64;
                Ok(self)
            }
        }
    }

    pub fn finish(self) -> Result<(), AssetArchiveError> {
        let header = AssetArchiveHeader::new(self.written_blobs);
        let mut writer = self.writer;
        let cbor_header = serde_cbor::to_vec(&header)?;
        let compressed_header = lz4_flex::compress(&cbor_header);
        writer.write(&compressed_header)?;
        let uncompressed_size_bytes = u64::to_le_bytes(cbor_header.len() as u64);
        writer.write(&uncompressed_size_bytes)?;
        let compressed_size_bytes = u64::to_le_bytes(compressed_header.len() as u64);
        writer.write(&compressed_size_bytes)?;
        writer.flush()?;
        Ok(())
    }
}
