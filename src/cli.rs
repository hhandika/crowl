use clap::{Parser, Subcommand};
use std::path::Path;

use crate::file;
use crate::md5::Md5;

#[derive(Parser)]
#[command(author, version, about, long_about)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Clone, Subcommand)]
enum Commands {
    #[command(about = "Find a file")]
    Find {
        #[arg(value_name = "FILE", help = "File to check")]
        regex: String,
    },

    #[command(about = "Compare MD5")]
    MD5 {
        #[arg(
            long = "input",
            short = 'i',
            value_name = "PATH",
            help = "File to check"
        )]
        input: String,
        #[arg(
            long = "regex",
            short = 'r',
            value_name = "REGEX",
            help = "Regex to match files"
        )]
        regex: String,
    },
}

pub fn parse_cli() {
    let args = Cli::parse();
    match args.command {
        Commands::Find { regex } => {
            file::walk_dir(&regex);
        }
        Commands::MD5 { input, regex } => {
            Md5::new(Path::new(&input), &regex).match_md5();
        }
    }
}
