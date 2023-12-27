use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use makinilya_core::core::MakinilyaCore;

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
    // #[arg(short, long, verbatim_doc_comment)]
    // config: Option<PathBuf>,

    #[command(subcommand)]
    subcommand: SubCommands,
}

#[derive(Subcommand, Debug)]
enum SubCommands {
    /// Builds the makinilya manuscript
    #[command(verbatim_doc_comment, long_about = None)]
    Build(BuildArgs),

    /// Generates a new project
    #[command(verbatim_doc_comment, long_about = None)]
    New(NewArgs),
}

#[derive(Args, Debug)]
struct NewArgs {
    /// directory where project will be generated
    path: Option<PathBuf>,
}

#[derive(Args, Debug)]
struct BuildArgs {
    /// directory that contains the manifest
    path: Option<PathBuf>,
}

fn main() {
    let args = Cli::parse();

    match args.subcommand {
        SubCommands::Build(build_args) => {
            let path = build_args.path.unwrap_or("./Config.toml".into());

            match MakinilyaCore::build(path) {
                Err(error) => println!("{}", error),
                _ => (),
            }
        }
        SubCommands::New(new_args) => {
            let path = new_args.path.unwrap_or("./".into());

            match MakinilyaCore::new(path) {
                Err(error) => println!("{}", error),
                _ => (),
            }
        }
    }
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
