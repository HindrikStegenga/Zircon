use serde::*;

#[repr(u8)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ArchiveCompressionFormat {
    None = 0,
    ZSTD = 1,
}

#[repr(u8)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
pub enum AssetSerializationFormat {
    Binary = 0,
    Toml = 1,
    Yaml = 2,
    Unknown = 255,
}

impl From<&str> for AssetSerializationFormat {
    fn from(value: &str) -> Self {
        let value = value.to_lowercase();
        match value.as_str() {
            "bin" => AssetSerializationFormat::Binary,
            "toml" => AssetSerializationFormat::Toml,
            "yaml" => AssetSerializationFormat::Yaml,
            "yml" => AssetSerializationFormat::Yaml,
            _ => AssetSerializationFormat::Binary,
        }
    }
}
