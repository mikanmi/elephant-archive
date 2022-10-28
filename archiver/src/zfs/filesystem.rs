// Copyright (c) 2022 Patineboot.
// All rights reserved.
// Elephant Archive is licensed under BSD 2-Clause License.

use std::{sync::Mutex, collections::HashMap};

use once_cell::sync::OnceCell;

use super::{Snapshot, Driver};

pub struct Filesystem {
    name: String,
    snapshots: Vec<Snapshot>,
}

struct FilesystemAttribute {
    pools: Vec<String>,
    filesystems: Vec<String>,
}

static ATTRIBUTE_INSTANCE: OnceCell<FilesystemAttribute> = OnceCell::new();

impl FilesystemAttribute {
    fn global() -> &'static FilesystemAttribute {
        ATTRIBUTE_INSTANCE.get_or_init(|| FilesystemAttribute::new())
    }

    fn new() -> FilesystemAttribute {
        let driver = Driver::get_instance();
        let filesystem_names = driver.get_filesystems();

        // let iter = filesystem_names.iter();
        // let filesystems: Vec<Snapshot> = iter.map(|s| Filesystem::new(s)).collect();

        // let pools_name = filesystem_names.into_iter().filter
        //         (|x| !x.contains("/")).collect();
        let pools_name = vec!["dummy".to_string()];

        FilesystemAttribute {
            pools: pools_name,
            filesystems: filesystem_names,
        }
    }
}

struct SnapshotCache {
    filesystems: HashMap<String, Vec<Snapshot>>,
}

static CACHE_INSTANCE: OnceCell<Mutex<SnapshotCache>> = OnceCell::new();


impl Filesystem {
    pub fn exist(name: &str) -> bool {
        let attribute = FilesystemAttribute::global();

        attribute.filesystems.contains(&name.to_string())
    }

    pub fn from(name: &str) -> Result<Filesystem, String> {
        if !Self::exist(name) {
            let message = format!("'{}' filesystem is not exist on this machine.", name);
            elephant_log::error!("{message}");
            return Err(message);
        }

        Ok(Self::new(name))
    }

    pub fn get_pools() -> Vec<Filesystem> {
        let attribute = FilesystemAttribute::global();

        let iter = attribute.pools.iter();
        let pools: Vec<Filesystem> = iter.map(|s| Filesystem::new(s)).collect();

        pools
    }

    fn new(name: &str) -> Filesystem {
        let snapshots = Filesystem::assign_snapshots(name);

        let instance = Filesystem {
            name: name.to_string(),
            snapshots,
        };

        instance
    }

    fn assign_snapshots(filesystem: &str) -> Vec<Snapshot> {
        let driver = Driver::get_instance();
        let snapshot_names = driver.get_snapshots(filesystem);

        let iter = snapshot_names.iter();
        let snapshots: Vec<Snapshot> = iter.map(|s| Snapshot::new(s)).collect();

        snapshots
    }


    /// Get the snapshot 
    /// 
    pub fn get_snapshots(&self) -> &Vec<Snapshot> {
        &self.snapshots
    }
}
