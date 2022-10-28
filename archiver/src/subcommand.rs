// Copyright (c) 2022 Patineboot.
// All rights reserved.
// Elephant Archive is licensed under BSD 2-Clause License.

use crate::argument::{Argument, Command};
use crate::zfs::Filesystem;

pub struct ListSnapshotCommand;

pub trait SubCommand {

    fn accessibleFilesystem(&self) -> Result<(), String> {
        Err(String::from("SubCommand::accessibleFilesystem called"))
    }

    fn run(&self) -> Result<(), String> {
        elephant_log::error!("SubCommand::run called");

        Err(String::from("SubCommand::run called"))
    }
}

pub fn from(command: &Command) -> Box<dyn SubCommand> {

    let subcommand = match command {
        Command::ListSnapshot { .. } => {
            Box::new(ListSnapshotCommand {})
        },
        _ => { elephant_log::error!("Not Implemented yet"); panic!() },
    };

    subcommand
}


impl SubCommand for ListSnapshotCommand {

    fn run(&self) -> Result<(), String> {

        let args = Argument::global();
        let filesystems = &args.filesystem;

        for filesystem in filesystems {
            let filesystem = Filesystem::from(filesystem)?;
            let snapshots = filesystem.get_snapshots();

            elephant_log::info!("Snapshots: {:?}", snapshots);
        }

        Ok(())
    }
}
