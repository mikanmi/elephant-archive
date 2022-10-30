// Copyright (c) 2022 Patineboot.
// All rights reserved.
// Elephant Archive is licensed under BSD 2-Clause License.

use crate::argument::{Argument, Command};
use crate::zfs::Filesystem;

pub struct SnapshotCommand;
pub struct ListSnapshotCommand;

pub trait SubCommand {

    /// Confirm the ZFS filesystems of the command options are accessible or not.
    fn accessible_filesystem(&self) -> Result<(), String> {
        let args = Argument::global();
        let filesystems = &args.filesystem;

        let f = filesystems.iter().find(|x|!Filesystem::exist(x));
        let result = match f {
            Some(filesystem) => Err(format!("The '{filesystem}' ZFS filesystem is not found")),
            None => Ok(()),
        };

        result
    }

    fn launch(&self) -> Result<(), String> {
        self.accessible_filesystem()?;
        self.run()?;

        Ok(())
    }

    fn run(&self) -> Result<(), String> {
        elephant_log::error!("SubCommand::run called");
        Err(String::from("SubCommand::run called"))
    }
}

pub fn from(command: &Command) -> Box<dyn SubCommand> {

    elephant_log::error!("{:?}", command);

    let subcommand: Box<dyn SubCommand> = match command {
        Command::Snapshot { .. } => {
            Box::new( SnapshotCommand {} )
        },
        Command::ListSnapshot { .. } => {
            Box::new( ListSnapshotCommand {} )
        },
        _ => { elephant_log::error!("Not Implemented yet"); panic!() },
    };

    subcommand
}

impl SubCommand for SnapshotCommand {

    fn run(&self) -> Result<(), String> {

        let args = Argument::global();
        let filesystems = &args.filesystem;

        // display the snapshots every the filesystem.
        for filesystem in filesystems {
            let filesystem = Filesystem::from(filesystem)?;
            let snapshot = filesystem.take_snapshot();

            elephant_log::display!("Taken a snapshot: {}", snapshot.name());
        }

        Ok(())
    }
}

impl SubCommand for ListSnapshotCommand {

    fn run(&self) -> Result<(), String> {

        let args = Argument::global();
        let filesystems = &args.filesystem;

        // display the snapshots every the filesystem.
        for filesystem in filesystems {
            let filesystem = Filesystem::from(filesystem)?;
            let snapshots = filesystem.get_snapshots();

            elephant_log::display!("Snapshots: {:?}", snapshots);
        }

        Ok(())
    }
}
