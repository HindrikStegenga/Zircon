use std::fs::*;
use std::io::*;
use std::path::Path;

pub(crate) fn load_file_bin(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(bytes)
}
