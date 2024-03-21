use clap::Args;

/// Show the current time
#[derive(Args, Debug)]
#[command(name = "clock", about)]
pub struct ClockArgs {}
