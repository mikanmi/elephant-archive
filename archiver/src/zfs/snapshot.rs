// Copyright (c) 2022 Patineboot.
// All rights reserved.
// Elephant Archive is licensed under BSD 2-Clause License.

use chrono::Local;
use crate::configure;

#[derive(Debug)]
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
}
