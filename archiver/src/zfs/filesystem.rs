// Copyright (c) 2022 Patineboot.
// All rights reserved.
// Elephant Archive is licensed under BSD 2-Clause License.

use super::{Snapshot, Driver};

pub struct Filesystem {
    name: String,
    snapshots: Vec<Snapshot>,
}

impl Filesystem {
    pub fn new(name: &str) -> Filesystem {
        let snaps = Filesystem::assign_snapshots(name);

        let instance = Filesystem {
            name: name.to_string(),
            snapshots: snaps,
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
