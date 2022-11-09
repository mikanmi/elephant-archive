// Copyright (c) 2022 Patineboot.
// All rights reserved.
// Elephant Archive is licensed under BSD 2-Clause License.

use std::{sync::Mutex};
use once_cell::sync::Lazy;
use chrono::{Local, Duration, TimeZone};

use crate::configure;

use super::{Snapshot, Driver};

#[derive(Debug, Clone)]
struct FilesystemAttribute {
    filesystems: Vec<String>,
    snapshots: Vec<String>,
}

static ATTRIBUTE_INSTANCE: Lazy<Mutex<FilesystemAttribute>> = 
        Lazy::new(|| Mutex::new(FilesystemAttribute::new()));

impl FilesystemAttribute {
    fn new() -> FilesystemAttribute {
        let driver = Driver::get_instance();

        let filesystem_names = driver.get_filesystems();
        let snapshot_names = driver.get_snapshots();

        FilesystemAttribute {
            filesystems: filesystem_names,
            snapshots: snapshot_names,
        }
    }
}

struct FilesystemController;

impl FilesystemController {
    /// Get the FilesystemAttribute instance.
    /// The instance contains the attributes of the ZFS filesystem.
    fn global() -> FilesystemController {
        FilesystemController{}
    }

    fn exist(&self, filesystem: &str) -> bool {
        let fa = ATTRIBUTE_INSTANCE.lock().unwrap();
        let filesystems = fa.filesystems.clone();

        filesystems.contains(&filesystem.to_string())
    }

    fn get_snapshots(&self, filesystem: &str) -> Vec<String> {
        let fa = ATTRIBUTE_INSTANCE.lock().unwrap();
        let snapshots = fa.snapshots.clone();

        // filter `snapshots` with starting `name`.
        let into_iter = snapshots.into_iter();
        let filtered: Vec<String> = into_iter.filter(|x| x.starts_with(filesystem)).collect();

        filtered
    }

    fn add_snapshot(&mut self, snapshot: &str) {
        let mut attribute = ATTRIBUTE_INSTANCE.lock().unwrap();
        attribute.snapshots.push(snapshot.to_string());
    }

    fn destroy_snapshots(&mut self, destroys: &Vec<Snapshot>) {
        let mut attribute = ATTRIBUTE_INSTANCE.lock().unwrap();

        attribute.snapshots
                .retain(|s| destroys.iter().any(|d| d.name() != *s));
    }
}

#[derive(Debug, Clone)]
pub struct Filesystem {
    name: String,
    controller: SnapshotCollector,
}

impl Filesystem {
    /// Confirm the `name` ZFS filesystem exists or not.
    pub fn exist(name: &str) -> bool {
        let attribute = FilesystemController::global();

        attribute.exist(name)
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
        let controller = SnapshotCollector::new(name);

        let instance = Filesystem {
            name: name.to_string(),
            controller,
        };

        instance
    }

    #[allow(dead_code)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn snapshots(&self) -> Vec<Snapshot> {
        self.controller.snapshots()
    }

    /// Get the snapshots
    pub fn take_snapshot(&mut self) -> Snapshot {
        let snapshot = self.controller.take();
        snapshot
    }

    pub fn purge_snapshots(&mut self) -> Vec<Snapshot> {
        let destroys = self.controller.purge();
        destroys
    }

}

#[derive(Debug, Clone)]
pub struct SnapshotCollector {
    filesystem: String,
    snapshots: Vec<Snapshot>,
}

impl SnapshotCollector {
    pub fn new(filesystem: &str) -> SnapshotCollector {
        let attribute = FilesystemController::global();
        let snapshot_names = attribute.get_snapshots(filesystem);

        let iter = snapshot_names.iter();
        let snapshots: Vec<Snapshot> = iter.map(|s| Snapshot::new(s)).collect();

        SnapshotCollector {
            filesystem: filesystem.to_string(),
            snapshots,
        }
    }

    pub fn snapshots(&self) -> Vec<Snapshot> {
        self.snapshots.clone()
    }

    pub fn take(&mut self) -> Snapshot {
        let driver = Driver::get_instance();
        let filesystem = Snapshot::generate_name(&self.filesystem);
        driver.take_snapshot(&filesystem);

        let mut attribute = FilesystemController::global();
        attribute.add_snapshot(&filesystem);

        let snapshot = Snapshot::new(&filesystem);
        self.snapshots.push(snapshot.clone());

        snapshot
    }

    /// Purge snapshots.
    pub fn purge(&self) -> Vec<Snapshot> {
        elephant_log::trace!("purge start");

        let generation = Snapshot::generation(&self.snapshots);
        let mut destroys: Vec<Snapshot> = Vec::new();

        let offset = Duration::days(1);
        let mut middles = 
                self.find_frequent_snapshot(&generation.middle, offset);
        destroys.append(&mut middles);

        let offset = Duration::weeks(1);
        let mut olds = 
                self.find_frequent_snapshot(&generation.old, offset);
        destroys.append(&mut olds);

        let mut oldests = self.find_oldest_snapshot(
                &generation.old,
                configure::SNAPSHOT_KEEP_WEEKS);
        destroys.append(&mut oldests);

        for destroy in destroys.iter() {
            Driver.destroy_snapshot(&destroy.name());
        }

        let mut attribute = FilesystemController::global();
        attribute.destroy_snapshots(&destroys);

        elephant_log::trace!("purge end: {:?}", destroys);
        destroys
    }

    /// Destroy frequent `snapshots` that were taken between `interval`s.
    /// Return an array of snapshot instance destroyed.
    /// # Arguments
    /// - `snapshots` - An array of snapshot instance that will be destroyed.
    /// - `interval` - An interval time to keep `snapshots`.
    fn find_frequent_snapshot(&self, snapshots: &Vec<Snapshot>, interval: Duration) -> Vec<Snapshot> {
        elephant_log::trace!("start find frequent snapshot");

        let mut base = Local.timestamp_millis(0);

        // let destroys: Vec<Snapshot> = snapshots
        //         .iter()
        //         .filter(|s| {
        //             let dt = s.get_datetime();
        //             let keep = dt >= base;
        //             base = dt + interval;
        //             keep
        //         }
        // ).cloned().collect();

        let mut destroys = Vec::new();
        for snapshot in snapshots.iter() {
            let dt = snapshot.get_datetime();
            if dt >= base {
                // keep the current snapshot.
                base = dt + interval;
            }
            else {
                elephant_log::info!("find: {}, with base: {}", snapshot.name(), base);
                destroys.push(snapshot.clone());
            }
        }

        elephant_log::trace!("found frequent snapshot: {:?}", destroys);
        destroys
    }

    /// Destroy `snapshots` over the number of keeping snapshots.
    /// Return an array of snapshot instance destroyed.
    /// # Arguments
    /// - `snapshots` - An array of snapshot instance that will be destroyed.
    fn find_oldest_snapshot(&self, snapshots: &Vec<Snapshot>, number: i32) -> Vec<Snapshot> {
        elephant_log::trace!("find oldest snapshot");

        let mut destroys: Vec<Snapshot> = Vec::new();

        let count_i32 = snapshots.len() as i32 - number;
        if count_i32 <= 0 {
            return destroys;
        }

        let count_usize = count_i32 as usize;
        let mut index = 0;
        while index < count_usize {
            let snapshot = snapshots[index].clone();
            destroys.push(snapshot);
            index += 1;
        }

        elephant_log::trace!("found oldest snapshot: {:?}", destroys);
        destroys
    }
}
