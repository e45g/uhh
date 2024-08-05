use std::io::{self, Write};
use colored::Colorize;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    cursor::{MoveTo, self},
};



#[derive(Clone)]
pub struct Input {
    prompt: String,
    default: String,
    allow_empty: bool,
}

impl Input {
    pub fn new() -> Self {
        Self {
            prompt: "input".to_string(),
            default: "".to_string(),
            allow_empty: true,
        }
    }

    pub fn prompt(mut self, prompt: &str) -> Self {
        self.prompt = prompt.to_string();
        self
    }

    pub fn default(mut self, val: &str) -> Self {
        self.default = val.to_string();
        self
    }

    pub fn allow_empty(mut self, val: bool) -> Self {
        self.allow_empty = val;
        self
    }

    pub fn interact(self) -> io::Result<String> {
        //TODO move somewhere
        let prompt_prefix = "?".to_string().yellow().bold();
        let prompt_suffix = "›".to_string().bright_black();
        let success_prefix = "✔".to_string().green();
        let success_suffix = "·".to_string().bright_black();

        let current_line: u16 = cursor::position()?.1;
        let mut input = String::new(); 
        let mut stdout = io::stdout();
        
        enable_raw_mode()?;
        loop {
            print!("{} {} {} {}", prompt_prefix, self.prompt.bold(), prompt_suffix, input);
            stdout.flush()?;

            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char(c) => {
                        input.push(c);
                    }
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Enter => {
                        if self.allow_empty && input.is_empty() { break }
                        break;
                    }
                    _ => {}
                }
            }
            execute!(stdout, MoveTo(0, current_line), Clear(ClearType::FromCursorDown))?;
        }
        
        execute!(stdout, MoveTo(0, current_line), Clear(ClearType::CurrentLine))?;
        print!("{} {} {} {}", success_prefix, self.prompt.bold(), success_suffix, input.green());
        execute!(stdout, MoveTo(0, current_line+1))?;
        stdout.flush()?;
        disable_raw_mode()?;
        return Ok(input);
        
    }
}
