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
        let snaps = driver.get_snapshots(&filesystem);

        let mut snapshots: Vec<Snapshot> = Vec::new();
        let s = Snapshot::new(&snaps[0]);
        snapshots.push(s);


        // TODO: add one dummy entry.
        // let mut snapshots: Vec<Snapshot> = Vec::new();
        // let name = String::from("dummy");
        // let s = Snapshot::new(name);
        // snapshots.push(s);

        snapshots
    }

    /// Get the snapshot 
    /// 
    pub fn get_snapshots(&self) -> &Vec<Snapshot> {
        &self.snapshots
    }
}
