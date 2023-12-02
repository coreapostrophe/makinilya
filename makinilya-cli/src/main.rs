use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// ░█▄█░█▀█░█░█░▀█▀░█▀█░▀█▀░█░░░█░█░█▀█
/// ░█░█░█▀█░█▀▄░░█░░█░█░░█░░█░░░░█░░█▀█
/// ░▀░▀░▀░▀░▀░▀░▀▀▀░▀░▀░▀▀▀░▀▀▀░░▀░░▀░▀
///
/// An austere manuscript generator
///
/// Makinilya was built to be a minimal all-in-one application for writers.
/// It's founded upon the idea that most writing should have some programmatical
/// abstractions to boost authorial productivity.
#[derive(Parser, Debug)]
#[command(
    name = "Makinilya",
    version = "0.1.0",
    author = "coreapostrophe",
    verbatim_doc_comment,
    propagate_version = true
)]
struct Cli {
    /// Provides a path to the configuration.
    #[arg(short, long, verbatim_doc_comment)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    subcommand: SubCommands,
}

#[derive(Subcommand, Debug)]
enum SubCommands {
    /// Builds the makinilya manuscript
    #[command(verbatim_doc_comment, long_about = None)]
    Build {
        /// Provides an explicit path to the project
        #[arg(short, long)]
        path: Option<String>,
    },
}

#[derive(Parser, Debug)]
struct NewArgs {
    #[arg(short, long)]
    path: String,
}

fn main() {
    Cli::parse();
}

#[cfg(test)]
mod cli_test {
    use clap::CommandFactory;

    use crate::Cli;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert()
    }
}
