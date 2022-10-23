// Copyright (c) 2022 Patineboot.
// All rights reserved.
// Elephant Archive is licensed under BSD 2-Clause License.

mod arguments;
mod elephant_logger;
mod zfs;

use zfs::Filesystem;

fn main() {
    elephant_logger::ElephantLogger::init().unwrap();

    let cli = arguments::Arguments::get_options();


    let number = 9;

    println!("Hello, world!{}", number);

    log::error!("log error message");
    log::warn!("log warn message");
    log::info!("log info message");
    log::debug!("log debug message");
    log::trace!("log trace message");

    archive();

    log::info!("Finished Elephant Archive.");
}

fn archive() {
    let pool_name = "zfs_pool";

    let filesystem = Filesystem::new(pool_name);
    let snapshots = filesystem.get_snapshots();

    log::info!("Snapshots: {:?}", snapshots);
    log::info!("Snapshots: {:?}", snapshots[0]);
}
