// Copyright (c) 2022 Patineboot.
// All rights reserved.
// Elephant Archive is licensed under BSD 2-Clause License.

use crate::argument::{Argument, ArchiverCommand};
use crate::zfs::{Filesystem, Snapshot};

pub trait SubCommand {

    /// Confirm the ZFS filesystems of the command options are accessible or not.
    fn accessible_filesystem(&self) -> Result<(), String> {
        self.accessible_filesystem_default()
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

    /// Confirm the ZFS filesystems of the command options are accessible or not.
    fn accessible_filesystem_default(&self) -> Result<(), String> {
        let args = Argument::global();
        let filesystems = &args.filesystem;

        let f = filesystems.iter().find(|x|!Filesystem::exist(x));
        let result = match f {
            Some(filesystem) => Err(format!("The '{filesystem}' ZFS filesystem is not found")),
            None => Ok(()),
        };

        result
    }

}

pub fn from(command: &ArchiverCommand) -> Box<dyn SubCommand> {

    elephant_log::error!("{:?}", command);

    let subcommand: Box<dyn SubCommand> = match command {
        ArchiverCommand::Snapshot { .. } => {
            Box::new( SnapshotCommand {} )
        },
        ArchiverCommand::Purge { .. } => {
            Box::new( PurgeCommand {} )
        },
        ArchiverCommand::Show { .. } => {
            Box::new( ShowCommand {} )
        },
        _ => { elephant_log::error!("Not Implemented yet"); panic!() },
    };

    subcommand
}

pub struct ArchiveCommand;

impl SubCommand for ArchiveCommand {

    fn accessible_filesystem(&self) -> Result<(), String> {
        SubCommand::accessible_filesystem_default(self)?;

        let args = Argument::global();
        let archive = &args.archive;
        let result = if Filesystem::exist(archive)
            { Ok(()) } else 
            { Err(format!("The '{archive}' ZFS filesystem is not found")) };

        result
    }

    fn run(&self) -> Result<(), String> {
        let args = Argument::global();
        let fs_names = &args.filesystem;

        // display the snapshots every the filesystem.
        for fs_name in fs_names {
            let mut filesystem = Filesystem::from(fs_name)?;

        }

        Ok(())
    }
}

pub struct SnapshotCommand;

impl SubCommand for SnapshotCommand {

    fn run(&self) -> Result<(), String> {
        let args = Argument::global();
        let fs_names = &args.filesystem;

        // take a snapshot every the filesystems.
        for fs_name in fs_names {
            let mut filesystem = Filesystem::from(fs_name)?;
            let snapshot = filesystem.take_snapshot();
            elephant_log::display!("Taken a snapshot: {}", snapshot.name());
        }

        Ok(())
    }
}

pub struct PurgeCommand;

impl SubCommand for PurgeCommand {

    fn run(&self) -> Result<(), String> {
        let args = Argument::global();
        let fs_names = &args.filesystem;

        // purge some snapshots every the filesystems.
        for fs_name in fs_names {
            let mut filesystem = Filesystem::from(fs_name)?;
            let destroys = filesystem.purge_snapshots();
            elephant_log::display!("Destroy snapshots: {:?}", destroys);
        }

        Ok(())
    }
}

pub struct ShowCommand;

impl SubCommand for ShowCommand {

    fn run(&self) -> Result<(), String> {
        let args = Argument::global();
        let filesystems = &args.filesystem;

        // display the snapshots every the filesystems.
        for filesystem in filesystems {
            let filesystem = Filesystem::from(filesystem)?;

            let snapshots = filesystem.snapshots();
            let generation = Snapshot::generation(&snapshots);

            elephant_log::display!("Young snapshots:");
            for snapshot in generation.young {
                elephant_log::display!("{}", snapshot.name());
            }

            elephant_log::display!("Middle snapshots:");
            for snapshot in generation.middle {
                elephant_log::display!("{}", snapshot.name());
            }

            elephant_log::display!("Old snapshots:");
            for snapshot in generation.old {
                elephant_log::display!("{}", snapshot.name());
            }
        }

        Ok(())
    }
}
