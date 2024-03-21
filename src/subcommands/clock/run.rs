use crate::cli::subcmd::RunnableSubCmd;
use crate::subcommands::ClockArgs;
use chrono::Local;

impl RunnableSubCmd for ClockArgs {
    fn run(&self) -> Result<(), String> {
        let now = Local::now().format("%H:%M:%S");
        println!("{now}");
        Ok(())
    }
}
