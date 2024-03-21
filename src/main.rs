use clap::Parser;

use crate::cli::subcmd::{RunnableSubCmd, SubCmd};
use crate::cli::Cli;

mod cli;
mod common;
mod subcommands;

fn main() {
    let cli = Cli::parse();
    let out = match &cli.subcommand {
        Some(SubCmd::Countdown(args)) => args.run(),
        Some(SubCmd::Clock(args)) => args.run(),
        None => cli.run(),
    };

    match out {
        Ok(_) => std::process::exit(0),
        Err(error) => {
            eprintln!("Error:\n{}", error);
            std::process::exit(1);
        }
    }
}
