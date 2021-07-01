use crate::{
    archive::*,
    vfs::{physical_mount_point::*, *},
};
use std::{fs::File, path::PathBuf};

#[test]
fn test_vfs() {
    let mut vfs = VirtualFileSystem::default();
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("test_files/physical");
    let mut mount = VfsPhysicalMountPoint::new(&"test", &d).unwrap();
    let index = mount.asset_index().as_ref().unwrap();
    assert!(vfs.mount(mount));
}

#[test]
fn test_archive_builder() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("../tmp/");
    std::fs::create_dir_all(d.clone()).unwrap();

    d.push("tmp.mtra");
    let file = File::create(d.clone()).unwrap();
    let builder = AssetArchiveBuilder::new(file).unwrap();

    let random_data = (0..1_048_576)
        .into_iter()
        .map(|_| rand::random())
        .collect::<Vec<_>>();

    use crate::archive::AssetArchiveCompressionFormat::{None, LZ4};

    let result = builder
        .write_blob(&random_data, None)
        .unwrap()
        .write_blob(&random_data, LZ4)
        .unwrap()
        .finish();
    assert!(result.is_ok());

    let file = File::open(d).unwrap();
    let archive = AssetArchive::read_from_file(file).unwrap();

    let first_blob = archive.read_blob(&archive.header().assets()[0]).unwrap();
    let second_blob = archive.read_blob(&archive.header().assets()[1]).unwrap();
    assert_eq!(first_blob, random_data);
    assert_eq!(second_blob, random_data);
}
