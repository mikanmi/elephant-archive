// Copyright (c) 2022 Patineboot.
// All rights reserved.
// Elephant Archive is licensed under BSD 2-Clause License.

use once_cell::sync::OnceCell;

use super::{Snapshot, Driver};

struct FilesystemAttribute {
    filesystems: Vec<String>,
    snapshots: Vec<String>,
}

static ATTRIBUTE_INSTANCE: OnceCell<FilesystemAttribute> = OnceCell::new();

impl FilesystemAttribute {
    fn global() -> &'static FilesystemAttribute {
        ATTRIBUTE_INSTANCE.get_or_init(|| FilesystemAttribute::new())
    }

    fn new() -> FilesystemAttribute {
        let driver = Driver::get_instance();

        let filesystem_names = driver.get_filesystems();
        let snapshot_names = driver.get_snapshots();

        FilesystemAttribute {
            filesystems: filesystem_names,
            snapshots: snapshot_names,
        }
    }

    fn get_snapshots(&self, filesystem: &str) -> Vec<String> {
        let snapshots = self.snapshots.clone();

        // filter `snapshots` with starting `name`.
        let into_iter = snapshots.into_iter();
        let filtered: Vec<String> = into_iter.filter(|x| x.starts_with(filesystem)).collect();

        filtered
    }

    fn add(&self, snapshot: &str) {
        let mut snapshots = self.snapshots.clone();
        snapshots.push(snapshot.to_string());
    }

}

pub struct Filesystem {
    name: String,
    snapshots: Vec<Snapshot>,
}

impl Filesystem {

    /// Confirm the `name` ZFS filesystem exists or not.
    pub fn exist(name: &str) -> bool {
        let attribute = FilesystemAttribute::global();

        attribute.filesystems.contains(&name.to_string())
    }

    /// Make a ZFS filesystem instance from `name`.
    pub fn from(name: &str) -> Result<Filesystem, String> {
        if !Self::exist(name) {
            let message = format!("'{}' filesystem is not exist on this machine.", name);
            elephant_log::error!("{message}");
            return Err(message);
        }

        Ok(Self::new(name))
    }

    /// Create a ZFS filesystem instance from `name`.
    fn new(name: &str) -> Filesystem {
        let attribute = FilesystemAttribute::global();
        let snapshot_names = attribute.get_snapshots(name);

        let iter = snapshot_names.iter();
        let snapshots: Vec<Snapshot> = iter.map(|s| Snapshot::new(s)).collect();

        let instance = Filesystem {
            name: name.to_string(),
            snapshots,
        };

        instance
    }

    /// Get the snapshots
    pub fn get_snapshots(&self) -> &Vec<Snapshot> {
        &self.snapshots
    }

    /// Get the snapshots
    pub fn take_snapshot(&self) -> Snapshot {
        let driver = Driver::get_instance();
        let name = Snapshot::generate_name(&self.name);
        driver.take_snapshot(&name);

        let attr = FilesystemAttribute::global();
        attr.add(&name);

        Snapshot::new(&name)
    }

}
