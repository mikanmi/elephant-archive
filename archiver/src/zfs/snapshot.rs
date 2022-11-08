// Copyright (c) 2022 Patineboot.
// All rights reserved.
// Elephant Archive is licensed under BSD 2-Clause License.

use chrono::{Local, DateTime, Duration};

use crate::configure;

#[derive(Debug, Clone)]
pub struct Generation {
    pub young: Vec<Snapshot>,
    pub middle: Vec<Snapshot>,
    pub old: Vec<Snapshot>,
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

        datetime_local
    }

    pub fn generation(snapshots: &Vec<Snapshot>) -> Generation {
        elephant_log::trace!("get generation start");

        let hours_duration = Duration::hours(configure::SNAPSHOT_KEEP_HOURS as i64);
        let days_duration = Duration::days(configure::SNAPSHOT_KEEP_DAYS  as i64);

        let local: DateTime<Local> = Local::now();
        let hours_limit = local + hours_duration;
        let days_limit = local + days_duration;

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

        elephant_log::debug!("young: {:?}", young);
        elephant_log::debug!("middle: {:?}", middle);
        elephant_log::debug!("old: {:?}", old);
        Generation { young, middle, old }

    }
}
