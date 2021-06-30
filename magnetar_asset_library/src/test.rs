use crate::vfs::{physical_mount_point::*, *};
use std::path::PathBuf;

#[test]
fn test_vfs() {
    let mut vfs = MtrVFS::default();
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("test_files/physical");
    let mut mount = MtrVFSPhysicalMountPoint::new(&"test", &d).unwrap();
    let index = mount.asset_index().as_ref().unwrap();
    assert!(vfs.mount(mount));
}
