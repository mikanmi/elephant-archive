// Copyright (c) 2022 Patineboot.
// All rights reserved.
// Elephant Archive is licensed under BSD 2-Clause License.

use chrono::{Local, DateTime, Duration, TimeZone};

use crate::configure;
use super::Driver;

pub struct Generation {
    pub young: Vec<Snapshot>,
    pub middle: Vec<Snapshot>,
    pub old: Vec<Snapshot>,
}

#[derive(Debug, Clone)]
pub struct SnapshotMemory {
    snapshots: Vec<Snapshot>,
}

impl SnapshotMemory {
    pub fn new(snapshots: Vec<Snapshot>) -> SnapshotMemory {
        SnapshotMemory {
            snapshots,
        }
    }

    pub fn get_generation(&self) -> Generation {
        let hours_duration = Duration::hours(configure::SNAPSHOT_KEEP_HOURS);
        let days_duration = Duration::days(configure::SNAPSHOT_KEEP_DAYS);

        let local: DateTime<Local> = Local::now();
        let hours_limit = local + hours_duration;
        let days_limit = local + days_duration;

        let snapshots = &self.snapshots;

        let young: Vec<Snapshot> = snapshots.iter()
                .filter(|s| s.get_datetime() < hours_limit)
                .cloned().collect();
        let middle: Vec<Snapshot> = snapshots.iter()
                .filter(|s| {
                    let dt = s.get_datetime();
                    dt >= hours_limit && dt < days_limit
                })
                .cloned().collect();
        let old: Vec<Snapshot> = snapshots.iter()
                .filter(|s| s.get_datetime() >= days_limit)
                .cloned().collect();

        Generation { young, middle, old }

    }

    /// Purge snapshots.
    pub fn purge(&self) {
        let mut generation = self.get_generation();

        let offset = Duration::hours(1);
        self.destroy(&mut generation.young, offset);

        let offset = Duration::days(1);
        self.destroy(&mut generation.middle, offset);

        self.destroy_oldest(&mut generation.old, configure::SNAPSHOT_KEEP_WEEKS);
    }

    /// Destroy `snapshots` with `offset`.
    /// Destroy frequent snapshots that were taken between offsets.
    fn destroy(&self, snapshots: &mut Vec<Snapshot>, offset: Duration) {

        let mut earliest = Local.timestamp_millis(i64::MIN);
        let mut keeps: Vec<bool> = Vec::new();

        for snapshot in snapshots.into_iter() {
            let dt = snapshot.get_datetime();
            if dt >= earliest {
                // keep the current snapshot.
                keeps.push(true);
                earliest = dt + offset;
            }
            else {
                keeps.push(false);
            }
        }

        let mut iter = keeps.iter();
        snapshots.retain(|s| {
            let keep = *iter.next().unwrap();
            if !keep {
                Driver.destroy_snapshot(&s.name());
            }
            keep
        });

    }

    /// Destroy `snapshots` over the number of keeping snapshots.
    fn destroy_oldest(&self, snapshots: &mut Vec<Snapshot>, number: i64) {
        let len: i64 = snapshots.len() as i64;
        let count = len - number;

        while count > 0 {
            let snapshot = snapshots.pop().unwrap();
            Driver.destroy_snapshot(&snapshot.name());
        }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct Snapshot {
   name: String 
}

impl Snapshot {
    pub fn generate_name(name: &str) -> String {
        let prefix = configure::SNAPSHOT_PREFIX;
        let datetime = Local::now()
                .format("%Y-%m%d-%H%M%S");

        format!("{name}@{prefix}-{datetime}")
    }

    pub fn new(name: &str) -> Snapshot {
        Snapshot {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn get_datetime(&self) -> DateTime<Local> {
        let prefix = configure::SNAPSHOT_PREFIX;

        let now = Local::now();

        let short_name = self.name.split("@").last().unwrap();
        let short_name_tz = format!("{}{}", short_name, now.offset());

        let datetime_offset = DateTime::parse_from_str(
                &short_name_tz,
                &format!("{prefix}-%Y-%m%d-%H%M%S%z")
        ).unwrap();
        let datetime_local: DateTime<Local> = DateTime::from(datetime_offset);
        
        elephant_log::debug!("{:?}", datetime_local);

        datetime_local
    }
}
