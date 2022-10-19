// Copyright (c) 2022 Patineboot.
// All rights reserved.
// Elephant Archive is licensed under BSD 2-Clause License.

pub struct Filesystem;

use super::snapshot::Snapshot;

impl Filesystem {

    pub fn init() -> Filesystem {
        let instance = Filesystem {
        };
        instance
    }

    /// Get the snapshot 
    /// 
    pub fn get_snapshots(&self) -> Vec<Snapshot> {
        let name = String::from("dummy");

        let s = Snapshot::new(name);
        let vector = vec![s];

        vector
    }
}
