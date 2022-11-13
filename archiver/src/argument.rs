// Copyright (c) 2022 Patineboot.
// All rights reserved.
// Elephant Archive is licensed under BSD 2-Clause License.

// use std::path::PathBuf;

use once_cell::sync::OnceCell;
use clap::{Parser, Subcommand};

#[derive(Debug)]
pub struct Argument {
    pub command: Command,
    pub filesystem: Vec<String>,
    pub archive: String,
    pub progress: bool,
    pub verbose: u8,
    pub dryrun: bool,
}

#[derive(Debug)]
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Program {
    /// Optional name to operate on
    // pub name: Option<String>,

    /// Sets a custom config file
    // #[arg(short, long, value_name = "FILE")]
    // pub config: Option<PathBuf>,

    /// Turn debugging information on
    // #[arg(short, long, action = clap::ArgAction::Count)]
    // pub debug: u8,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Subcommand)]
pub enum Command {
    /// Archive one or more ZFS filesystems to another ZFS filesystem.
    Archive {
        /// The names of one or more ZFS filesystems.
        #[clap(required = true)]
        filesystem: Vec<String>,

        /// The name of ZFS filesystem archiving original ZFS filesystems.
        #[arg(short, long, required = true)]
        archive: String,

        /// Show the progress of archiving ZFS filesystems.
        #[arg(short, long, default_value_t = false)]
        progress: bool,

        /// Print verbose information running on the program.
        #[arg(short, long, action = clap::ArgAction::Count)]
        verbose: u8,

        /// Run the program under no changes.
        #[arg(short, long, default_value_t = false)]
        dryrun: bool,
    },
    /// TODO: implementation
    Restore {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    /// TODO: implementation
    Diff {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    /// Take a snapshot on a ZFS filesystem.
    Snapshot {
        /// The names of one or more ZFS filesystems.
        #[clap(required = true)]
        filesystem: Vec<String>,

        /// Print verbose information running on the program. 
        #[arg(short, long, action = clap::ArgAction::Count)]
        verbose: u8,

        /// Run the program under no changes.
        #[arg(short, long, default_value_t = false)]
        dryrun: bool,
    },
    /// Purge some existing snapshots on ZFS filesystems.
    Purge {
        /// The names of one or more ZFS filesystems.
        #[clap(required = true)]
        filesystem: Vec<String>,

        /// Print verbose information running on the program. 
        #[arg(short, long, action = clap::ArgAction::Count)]
        verbose: u8,

        /// Run the program under no changes.
        #[arg(short, long, default_value_t = false)]
        dryrun: bool,
    },
    /// Show the existing snapshots on ZFS filesystems.
    Show {
        /// The names of one or more ZFS filesystems.
        #[clap(required = true)]
        filesystem: Vec<String>,

        /// Print verbose information running on the program. 
        #[arg(short, long, action = clap::ArgAction::Count)]
        verbose: u8,

        /// Run the program under no changes.
        #[arg(short, long, default_value_t = false)]
        dryrun: bool,
    },
}


static SINGLETON_INSTANCE: OnceCell<Argument> = OnceCell::new();

impl Argument {
    pub fn global() -> &'static Argument {
        let singleton = SINGLETON_INSTANCE.get_or_init(|| Argument::new());

        singleton
    }

    pub fn new() -> Argument {
        let program = Program::parse();

        println!("{:?}", program);

        let argument = match &program.command {
            Command::Archive { filesystem, archive, progress,
                     verbose, dryrun } => {
                Argument {
                    command: program.command.clone(),
                    filesystem: filesystem.clone(),
                    archive: archive.clone(),
                    progress: progress.clone(),
                    verbose: verbose.clone(),
                    dryrun: dryrun.clone(),
                }
            },
            Command::Snapshot { filesystem, verbose, dryrun } |
            Command::Purge { filesystem, verbose, dryrun } |
            Command::Show { filesystem, verbose, dryrun }
            => {
                Argument {
                    command: program.command.clone(),
                    filesystem: filesystem.clone(),
                    archive: "Not expected".to_string(),
                    progress: false,
                    verbose: verbose.clone(),
                    dryrun: dryrun.clone(),
                }
            },
            _ => {panic!("not implemented yet.")}
        };

        argument
    }
}
