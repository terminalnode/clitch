use crate::subcommands::{ClockArgs, CountdownArgs};
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum SubCmd {
    Countdown(CountdownArgs),
    Clock(ClockArgs),
}

pub trait RunnableSubCmd {
    fn run(&self) -> Result<(), String>;
}
