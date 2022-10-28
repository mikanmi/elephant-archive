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
    /// TODO: implementation
    Archive {
        /// lists test values
        #[arg(short, long)]
        list: bool,
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
    /// Take a snapshot and purge some existing snapshots on a ZFS filesystem.
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
    /// Show the existing snapshots on a ZFS filesystem.
    ListSnapshot {
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
            Command::ListSnapshot { filesystem, verbose, dryrun } => {
                Argument {
                    command: program.command.clone(),
                    filesystem: filesystem.clone(),
                    verbose: verbose.clone(),
                    dryrun: dryrun.clone(),
                }
            },
            _ => {panic!("not implemented yet.")}
        };

        argument
    }
}


