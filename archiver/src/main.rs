// Copyright (c) 2022 Patineboot.
// All rights reserved.
// Elephant Archive is licensed under BSD 2-Clause License.

mod arguments;
mod zfs;

use zfs::Filesystem;
use arguments::{Arguments, Command};

fn main() {

    elephant_log::Logger::init(elephant_log::Level::Trace);
    let args = Arguments::global();
    let command = &args.command;

    match command {
        Command::ListSnapshot { .. } => {
            archive()
        }
        _ => elephant_log::error!("Not Implemented yet"),
    }

    elephant_log::error!("log error message");
    elephant_log::warn!("log warn message");
    elephant_log::info!("log info message");
    elephant_log::debug!("log debug message");
    elephant_log::trace!("log trace message");
    elephant_log::display!("log display message");

    archive();

    elephant_log::display!("Finished Elephant Archive.");
}

fn archive() {
    let pool_name = "zfs_pool";

    let filesystem = Filesystem::new(pool_name);
    let snapshots = filesystem.get_snapshots();

    elephant_log::info!("Snapshots: {:?}", snapshots);
}
