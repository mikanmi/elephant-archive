// Copyright (c) 2022 Patineboot.
// All rights reserved.
// Elephant Archive is licensed under BSD 2-Clause License.

use std::process::Command;

use log::debug;

pub struct Driver;

static DRIVER_INSTANCE: Driver = Driver;

const ZFS_LIST_SNAPSHOT: &str = "zfs list -H -s creation -o name -t snapshot";

impl Driver {
    pub fn get_instance() -> &'static Driver {
        &DRIVER_INSTANCE
    }

    pub fn get_snapshots(&self, filesystem: &str) -> Vec<String> {

        let cl = format!("{} {}", ZFS_LIST_SNAPSHOT, filesystem);
        let stdout = self.spawn(&cl);

        vec![stdout]
    }

    fn spawn(&self, command_line: &str) -> String {
        debug!("spawn: {}", command_line);

        let mut split = command_line.split_whitespace();
        let command = split.next().unwrap();
        let arguments: Vec<&str> = split.collect();
        // let arguments = split.as_str();

        let output = Command::new(command)
                            .args(arguments)
                            .output()
                            .expect("Failed to execute command");

        let stdout = String::from_utf8(output.stdout).unwrap();

        debug!("stdout: {}", stdout);

        stdout
    }
}
