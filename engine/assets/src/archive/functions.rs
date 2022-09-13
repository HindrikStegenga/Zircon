use crate::*;
use async_recursion::*;
use std::path::Path;
use tokio::fs::*;
use tokio::io::*;

pub async fn create_archive_from_directory(
    initial_prefix: impl AsRef<str>,
    path: impl AsRef<Path>,
    out: impl AsRef<Path>,
    version: u16,
    compression_format: ArchiveCompressionFormat,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let directory = std::fs::read_dir(path)?;
    let out_file = File::create(out).await?;
    let mut buf_writer = BufWriter::new(out_file);
    let mut builder = ArchiveBuilder::new(&mut buf_writer).await?;
    add_dir_to_archive(
        String::from(initial_prefix.as_ref()),
        directory,
        &mut builder,
        version,
        compression_format,
    )
    .await?;
    builder.finish(uuid::Uuid::new_v4()).await?;
    buf_writer.flush().await.unwrap();
    Ok(())
}

#[async_recursion]
pub async fn add_dir_to_archive<F: AsyncWriteExt + Unpin + Send>(
    current_subdir: String,
    dir: std::fs::ReadDir,
    builder: &mut ArchiveBuilder<'_, F>,
    version: u16,
    compression_format: ArchiveCompressionFormat,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
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
            let fs_dir = match std::fs::read_dir(dir.path()) {
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

            let fname = if current_subdir.is_empty() {
                String::from(fname)
            } else {
                String::from(current_subdir.clone()) + &(String::from(".") + &fname)
            };

            files.push((dir, name, md, fname))
        }
    });

    for (fs_dir, _, name, _) in sub_dirs {
        let current_subdir = if current_subdir.is_empty() {
            String::from(name)
        } else {
            String::from(current_subdir.clone()) + &(String::from(".") + &name)
        };

        match add_dir_to_archive(current_subdir, fs_dir, builder, version, compression_format).await
        {
            Ok(v) => v,
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    if files.is_empty() {
        return Ok(());
    }

    for (dir, name, _, fname) in files {
        let buf = match std::fs::read(dir.path()) {
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

        if let Err(e) = builder
            .write_file(
                &fname,
                format.as_str().into(),
                &buf,
                version,
                compression_format,
            )
            .await
        {
            println!("Could not add file: {} - {}", e, name);
        }
    }
    Ok(())
}
