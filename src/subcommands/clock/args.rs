use clap::Args;

/// Show the current time
#[derive(Args, Debug)]
#[command(name = "clock", about)]
pub struct ClockArgs {
    /// If set, print the remaining time every second
    #[arg(short = 'c', long = "continuous")]
    pub continuous: bool,

    /// If set in combination with -c / --continuous, overwrite the previous line instead of printing a new line
    #[arg(short = 'o', long = "overwrite")]
    pub overwrite: bool,
}
