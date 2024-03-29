// Copyright (c) 2022 Patineboot.
// All rights reserved.
// Elephant Archive is licensed under BSD 2-Clause License.

use std::process::{Command, Stdio};

pub struct Driver;

static DRIVER_INSTANCE: Driver = Driver;

/// Command Line: show ZFS filesystems on this machine.
const ZFS_LIST_FILESYSTEM: &str = "zfs list -H -o name -t filesystem";

/// Command Line: show snapshots on this machine.
const ZFS_LIST_SNAPSHOT: &str = "zfs list -H -s creation -o name -t snapshot";

/// Command Line: take a snapshot recursively on a ZFS filesystem.
const ZFS_TAKE_SNAPSHOT: &str = "zfs snapshot -r";

/// Command Line: destroy a snapshot recursively on a ZFS filesystem.
const ZFS_DESTROY_SNAPSHOT: &str = "zfs destroy -r";


impl Driver {
    pub fn get_instance() -> &'static Driver {
        &DRIVER_INSTANCE
    }

    /// Get all of the filesystems on this machine.
    pub fn get_filesystems(&self) -> Vec<String> {
        let cl = format!("{ZFS_LIST_FILESYSTEM}");
        let stdout = self.spawn(&cl);

        let lines = stdout.lines();
        let filesystems = lines.map(|s| s.to_string()).collect();

        filesystems
    }

    /// Get all of the snapshots on this machine.
    pub fn get_snapshots(&self) -> Vec<String> {
        let cl = format!("{ZFS_LIST_SNAPSHOT}");
        let stdout = self.spawn(&cl);

        let lines = stdout.lines();
        let snapshots = lines.map(|s| s.to_string()).collect();

        snapshots
    }

    /// Take the snapshot named with `snapshot`.
    /// `take_snapshot` function must be called by the root user.
    pub fn take_snapshot(&self, snapshot: &str) {
        let cl = format!("{ZFS_TAKE_SNAPSHOT} {snapshot}");
        self.spawn(&cl);
    }

    /// Destroy the snapshot named with `snapshot`.
    /// `destroy_snapshot` function must be called by the root user.
    pub fn destroy_snapshot(&self, snapshot: &str) {
        let cl = format!("{ZFS_DESTROY_SNAPSHOT} {snapshot}");
        self.spawn(&cl);
    }

    /// Execute a command line involving a program and arguments.
    /// `command_line` is a command line with a program followed 
    /// by arguments separated with whitespace.
    fn spawn(&self, command_line: &str) -> String {
        elephant_log::info!("spawn: {command_line}");

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

        let child = command.spawn().expect("Failed to execute child");
        let output = child.wait_with_output().expect("Failed to wait on child");
        if !output.status.success() {
            let exit = match output.status.code() {
                Some(code) => format!("Exited with status code: {code}"),
                None            => format!("Process terminated by signal"),
            };
            let stderr = String::from_utf8(output.stderr).unwrap();

            elephant_log::error!("{exit}, See more details =====>");
            elephant_log::error!("Command Line: '{command_line}', stderr is on the next line:\n{stderr}");
            panic!();
        }


        let stdout = String::from_utf8(output.stdout).unwrap();

        elephant_log::debug!("stdout: {stdout}");
        stdout
    }
}
