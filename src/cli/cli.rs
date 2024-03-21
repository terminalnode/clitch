use crate::cli::subcmd::{RunnableSubCmd, SubCmd};
use crate::subcommands::ClockArgs;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "clitch", about, version, propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Option<SubCmd>,

    #[command(flatten)]
    pub args: ClockArgs,
}

impl RunnableSubCmd for Cli {
    fn run(&self) -> Result<(), String> {
        self.args.run()
    }
}
