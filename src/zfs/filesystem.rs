// Copyright (c) 2022 Patineboot.
// All rights reserved.
// Elephant Archive is licensed under BSD 2-Clause License.

use super::Snapshot;

pub struct Filesystem {
    snapshots: Vec<Snapshot>
}

impl Filesystem {

    pub fn init() -> Filesystem {
        let snapshots: Vec<Snapshot> = Vec::new();
        let mut instance = Filesystem {
            snapshots
        };

        // TODO: add one dummy entry.
        let name = String::from("dummy");
        let s = Snapshot::new(name);
        instance.snapshots.push(s);

        instance
    }

    /// Get the snapshot 
    /// 
    pub fn get_snapshots(&self) -> &Vec<Snapshot> {
        &self.snapshots
    }
}
