// Copyright (c) 2022 Patineboot.
// All rights reserved.
// Elephant Archive is licensed under BSD 2-Clause License.

use std::process::{Command, Stdio};

use log::{error, debug};

pub struct Driver;

static DRIVER_INSTANCE: Driver = Driver;

const ZFS_LIST_SNAPSHOT: &str = "zfs list -H -s creation -o name -t snapshot";

impl Driver {
    pub fn get_instance() -> &'static Driver {
        &DRIVER_INSTANCE
    }

    pub fn get_snapshots(&self, filesystem: &str) -> Vec<String> {

        let cl = format!("{ZFS_LIST_SNAPSHOT} {filesystem}");
        let stdout = self.spawn(&cl);

        let lines = stdout.lines();
        let snapshots = lines.map(|s| s.to_string()).collect();

        snapshots
    }

    /// Execute a command line involving a program and arguments.
    /// `command_line` is a command line with a program followed 
    /// by arguments separated with whitespace.
    fn spawn(&self, command_line: &str) -> String {
        debug!("spawn: {command_line}");

        let mut split = command_line.split_whitespace();
        let program = split.next().unwrap();
        // SplitWhitespace.as_str() is not stable.
        // So, I use collect() instead of as_str().
        // let arguments = split.as_str();
        let arguments: Vec<&str> = split.collect();

        let mut command = Command::new(program);
        command.args(arguments);
        command.stdin(Stdio::null());
        command.stderr(Stdio::piped());
        command.stdout(Stdio::piped());
        
        let child = command.spawn().expect("failed to execute child");
        let output = child.wait_with_output().expect("Failed to wait on child");
        if !output.status.success() {
            error!("Spawn: {command_line}");
            match output.status.code() {
                Some(code) => error!("Exited with status code: {code}"),
                None            => error!("Process terminated by signal"),
            }
            let stderr = String::from_utf8(output.stderr).unwrap();
            error!("Error messages is the following ===>");
            error!("{stderr}");
            panic!();
        }


        let stdout = String::from_utf8(output.stdout).unwrap();

        debug!("stdout: {stdout}");
        stdout
    }
}
