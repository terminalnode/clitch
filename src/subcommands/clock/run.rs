use std::io::stdout;
use std::thread::sleep;
use std::time::Duration;

use chrono::{Local, Timelike};
use crossterm::cursor::MoveLeft;
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};

use crate::cli::subcmd::RunnableSubCmd;
use crate::subcommands::ClockArgs;

const FMT: &str = "%H:%M:%S";

impl ClockArgs {
    fn get_text(&self) -> (String, u32) {
        let datetime = Local::now();
        let nanos = 1_000_000_000 - datetime.time().nanosecond();
        let now = datetime.format(FMT).to_string();
        (now, nanos)
    }
}

impl RunnableSubCmd for ClockArgs {
    fn run(&self) -> Result<(), String> {
        if self.continuous {
            loop {
                let (text, nanos) = self.get_text();
                let execution = if self.overwrite {
                    execute!(
                        stdout(),
                        Clear(ClearType::CurrentLine),
                        MoveLeft(100),
                        Print(text)
                    )
                } else {
                    execute!(stdout(), Print(text), Print("\n"))
                };

                execution.or_else(|x| Err(x.to_string()))?;
                sleep(Duration::from_nanos(1 + nanos as u64));
            }
        } else {
            let (text, _) = self.get_text();
            println!("{text}");
        }
        Ok(())
    }
}
