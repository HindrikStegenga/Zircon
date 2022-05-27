use crate::archive::AssetArchiveCompressionFormat;
use crate::archive::{AssetArchive, AssetArchiveBuilder, AssetArchiveMountPointBuilder};
use std::fs;
use std::fs::*;
use std::io::*;
use std::path::Path;

pub(crate) fn load_file_bin(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(bytes)
}

pub fn archive_directory(
    path: impl AsRef<Path>,
    base_mount_point: impl AsRef<str>,
    out: impl AsRef<Path>,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let directory = fs::read_dir(path)?;
    let out_file = File::create(out)?;
    let archive = AssetArchiveBuilder::new(out_file)?;
    let archive = match add_dir_to_archive(directory, archive, base_mount_point, 0, true) {
        Ok(v) => v,
        Err((a, e)) => {
            println!("Error occured: {}", e);
            a
        }
    };
    archive.finish()?;
    Ok(())
}

pub fn add_dir_to_archive(
    dir: ReadDir,
    mut builder: AssetArchiveBuilder,
    mount_point: impl AsRef<str>,
    version: u64,
    compressed: bool,
) -> std::result::Result<AssetArchiveBuilder, (AssetArchiveBuilder, Box<dyn std::error::Error>)> {
    let mount_point = mount_point.as_ref();

    let mut sub_dirs = vec![];
    let mut files = vec![];

    dir.filter_map(|e| match e {
        Ok(v) => Some(v),
        Err(_) => None,
    })
    .filter_map(|d| {
        let name = match d.file_name().to_str() {
            Some(v) => String::from(v),
            None => return None,
        };
        Some((d, name))
    })
    .filter_map(|(d, n)| {
        let md = match d.metadata() {
            Ok(v) => v,
            Err(_) => return None,
        };
        Some((d, n, md))
    })
    .for_each(|(dir, name, md)| {
        if md.is_dir() {
            let fs_dir = match fs::read_dir(dir.path()) {
                Ok(v) => v,
                Err(_) => return,
            };

            sub_dirs.push((fs_dir, dir, name, md));
        } else if md.is_file() {
            let fname = match dir.path().file_stem() {
                Some(v) => match v.to_str() {
                    Some(v) => String::from(v),
                    None => return,
                },
                None => return,
            };

            files.push((dir, name, md, fname))
        }
    });

    for (fs_dir, _, name, _) in sub_dirs {
        let sub_mnt_point = if mount_point.is_empty() {
            String::from(name)
        } else {
            String::from(mount_point) + &(String::from(".") + &name)
        };

        builder = match add_dir_to_archive(fs_dir, builder, sub_mnt_point, version, compressed) {
            Ok(v) => v,
            Err((a, e)) => {
                println!("Error: {}", e);
                a
            }
        }
    }

    if files.is_empty() {
        return Ok(builder);
    }

    let mut mnt_point = match builder.add_mount_point(mount_point, version) {
        Ok(v) => v,
        Err((a, e)) => return Err((a, Box::from(e))),
    };

    for (dir, name, _, fname) in files {
        let buf = match fs::read(dir.path()) {
            Ok(v) => v,
            Err(e) => {
                println!("Could not read file: {} - {}", e, name);
                continue;
            }
        };
        let format = match dir.path().extension() {
            Some(v) => match v.to_str() {
                Some(v) => String::from(v),
                None => String::from(""),
            },
            None => String::from(""),
        };

        mnt_point = match mnt_point.write_file(&fname, format, &buf, {
            if compressed {
                AssetArchiveCompressionFormat::LZ4
            } else {
                AssetArchiveCompressionFormat::None
            }
        }) {
            Ok(v) => v,
            Err((a, e)) => {
                println!("Could not add file: {} - {}", e, name);
                a
            }
        }
    }

    builder = mnt_point.finish();
    Ok(builder)
}
