// Copyright (c) 2022 Patineboot.
// All rights reserved.
// Elephant Archive is licensed under BSD 2-Clause License.

mod configure;
mod argument;
mod subcommand;
mod zfs;

use argument::Argument;

fn main() {

    elephant_log::Logger::init(elephant_log::Level::Trace);

    let args = Argument::global();
    let command = &args.command;

    let subcommand = subcommand::from(command);
    let result = subcommand.launch();

    match result {
        Ok(()) => elephant_log::display!("Finished Elephant Archive."),
        Err(message) => elephant_log::error!("Error occurs {message}"),
    }

    elephant_log::error!("log error message");
    elephant_log::warn!("log warn message");
    elephant_log::info!("log info message");
    elephant_log::debug!("log debug message");
    elephant_log::trace!("log trace message");
    elephant_log::display!("log display message");

    elephant_log::display!("Finished Elephant Archive.");
}
