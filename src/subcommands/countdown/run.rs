use std::io::stdout;
use std::thread::sleep;
use std::time::Duration;

use crate::cli::subcmd::RunnableSubCmd;
use crossterm::cursor::MoveLeft;
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};

use crate::subcommands::countdown::time_from_now::TimeFromNow;
use crate::subcommands::CountdownArgs;

impl RunnableSubCmd for CountdownArgs {
    fn run(&self) -> Result<(), String> {
        let target = self.get_target()?;

        if self.continuous {
            let mut first = true;

            loop {
                let remaining = TimeFromNow::from(target)?;
                let formatted = remaining.formatted(self.verbose && first);

                let execution = if self.overwrite && !first {
                    execute!(
                        stdout(),
                        Clear(ClearType::CurrentLine),
                        MoveLeft(100),
                        Print(formatted)
                    )
                } else {
                    first = false;
                    if self.overwrite {
                        execute!(stdout(), Print(formatted))
                    } else {
                        execute!(stdout(), Print(formatted), Print("\n"))
                    }
                };

                execution.or_else(|x| Err(x.to_string()))?;
                sleep(Duration::from_millis(remaining.millis));
            }
        } else {
            let remaining = TimeFromNow::from(target)?;
            println!("{}", remaining.formatted(self.verbose));
        }

        Ok(())
    }
}
