// Copyright (c) 2022 Patineboot.
// All rights reserved.
// Elephant Archive is licensed under BSD 2-Clause License.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

pub struct Arguments;

impl Arguments {
    pub fn get_options() -> String {
        let cli = Cli::parse();

        String::from("aaa")
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
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
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    /// Show the existing snapshots on a ZFS filesystem.
    ListSnapshot {

        /// The names of one or more ZFS filesystems.
        filesystem: Vec<String>,

        /// Turn debugging information on
        #[arg(short, long, action = clap::ArgAction::Count)]
        debug: u8,
    },
}

