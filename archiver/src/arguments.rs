// Copyright (c) 2022 Patineboot.
// All rights reserved.
// Elephant Archive is licensed under BSD 2-Clause License.

// use std::path::PathBuf;

use once_cell::sync::OnceCell;
use clap::{Parser, Subcommand};

#[derive(Debug)]
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
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
        verbose: u8,

        /// Turn debugging information on
        #[arg(short, long, action = clap::ArgAction::Count)]
        dryrun: u8,
    },
}


static INSTANCE: OnceCell<Arguments> = OnceCell::new();

impl Arguments {
    pub fn global() -> &'static Arguments {
        let instance = INSTANCE.get_or_init(|| Arguments::new());

        instance
    }

    pub fn new() -> Arguments {
        let args = Arguments::parse();

        args
    }
}


