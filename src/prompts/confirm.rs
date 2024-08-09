use std::io::{self, Write};
use colored::Colorize;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    cursor::{Hide, Show, MoveTo, self},
};

#[derive(Clone)]
pub struct Confirm {
    prompt: String,

}

impl Confirm {
    pub fn new() -> Self {
        Self {
            prompt: "Confirm".to_string()
        }
    }

    pub fn interact(&self) -> io::Result<bool>{
        //TODO move this somewhere else 
        let prompt_prefix = "?".to_string().yellow().bold();
        let success_prefix = "✔".to_string().green();
        let success_suffix = "·".to_string().bright_black();

        let mut confirmed: bool = false;
        let mut stdout = io::stdout();
        let current_line: u16 = cursor::position()?.1;

        enable_raw_mode()?;

        loop {
            execute!(stdout, Hide, MoveTo(0, current_line))?;
            print!("{} {} {}", prompt_prefix, self.prompt.bold(), "(y/n)".bright_black());
            stdout.flush()?;

            //TODO refactor
            if let Event::Key(key_event) = event::read()? {
                if let KeyCode::Char(c) = key_event.code {
                    if c == 'y' {
                        confirmed = true;
                        break;
                    }
                    if c == 'n' {
                        confirmed = false;
                        break;
                    }
                }
            }

        }
        execute!(stdout, Show, MoveTo(0, current_line), Clear(ClearType::FromCursorDown))?;
        print!("{} {} {} {}", success_prefix, self.prompt.bold(), success_suffix, if confirmed { "yes".green() } else { "no".green() });
        execute!(stdout, cursor::MoveToNextLine(1))?;
        stdout.flush()?;
        disable_raw_mode()?;
        Ok(confirmed)
    }


}
