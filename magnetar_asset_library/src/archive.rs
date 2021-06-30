use serde::*;
use std::fs::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct MtrArchive {}

#[derive(Debug, Serialize, Deserialize)]
pub struct MtrArchiveHeader {}

#[derive(Debug, Serialize, Deserialize)]
pub struct MtrArchiveFileHeader {
    offset: usize,
    size: usize,
}
