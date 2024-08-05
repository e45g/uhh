use std::io::{self, Write};
use colored::Colorize;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    cursor::{MoveTo, Hide, Show, self},
};

#[derive(Clone)]
pub struct FuzzySelect{
    items: Vec<String>,
    prompt: String,
    max_mistake_ratio: f32,
}

impl FuzzySelect{
    pub fn new() -> Self {
        Self {
            items: Vec::new(), 
            prompt: String::from("Choose"),
            max_mistake_ratio: 0.5,
        }
    }
    pub fn items<T, I>(mut self, items: I) -> Self 
    where
        T: ToString,
        I: IntoIterator<Item = T>,
    {
        self.items.extend(items.into_iter().map(|item| item.to_string()));
        self

    }
    pub fn item(mut self, item: &str) -> Self{
        self.items.push(item.to_string());
        self
    }

    pub fn set_prompt(&mut self, prompt: String) {
        self.prompt = prompt;
    }
    
    pub fn interact(&self) -> io::Result<Option<usize>>{
        //TODO move this somewhere else 
        let prompt_prefix = "?".to_string().yellow().bold();
        let prompt_suffix = "›".to_string().bright_black();
        let success_prefix = "✔".to_string().green();
        let success_suffix = "·".to_string().bright_black();
        let active_item_prefix = "❯".to_string().green();

        let mut selected = 0;
        let mut filter: Vec<char> = Vec::new();
        let mut filtered_items: Vec<(usize, &String)> = self.items.iter().enumerate().collect();
        let mut stdout = io::stdout();
        let current_line: u16 = cursor::position()?.1;

        enable_raw_mode()?;

        loop {
            execute!(stdout, Hide, MoveTo(0, current_line))?;
            print!("{} {} {} {}", prompt_prefix, self.prompt.bold(), prompt_suffix, filter.iter().collect::<String>());



            for (i, (_original, item)) in filtered_items.iter().enumerate() {
                execute!(stdout, MoveTo(0, current_line + i as u16 + 1))?;
                if i == selected { print!("{} {}", active_item_prefix, item.cyan()) }
                else { print!("  {}", item); }
            }
            stdout.flush()?;

            //TODO refactor
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Up => {
                        if selected > 0 { selected -= 1; }
                        else { selected = filtered_items.len()-1}
                    }
                    KeyCode::Down | KeyCode::Tab => {
                        if selected < filtered_items.len()-1 { selected += 1; } 
                        else { selected = 0; }
                    }
                    KeyCode::Char(c) => {
                        selected = 0;
                        filter.push(c);
                        filtered_items = self.filter_items(&filter);
                    }
                    KeyCode::Backspace => {
                        selected = 0;
                        filter.pop();
                        if !filter.is_empty() { 
                            filtered_items = self.filter_items(&filter);
                        } else{
                            filtered_items = self.items.iter().enumerate().collect(); 
                        }

                    }
                    KeyCode::Enter => { 
                        if filtered_items.is_empty() { continue }
                        break;
                    }
                    KeyCode::Esc => { selected = self.items.len(); break; }
                    _ => {}
                }  
            }
            execute!(stdout, MoveTo(0, current_line), Clear(ClearType::FromCursorDown))?;
        }
        execute!(stdout, Show, MoveTo(0, current_line), Clear(ClearType::FromCursorDown))?;
        print!("{} {} {} {}", success_prefix, self.prompt.bold(), success_suffix, filtered_items[selected].1.green());
        execute!(stdout, cursor::MoveToNextLine(1))?;
        stdout.flush()?;
        disable_raw_mode()?;
        if selected < self.items.len() && filtered_items.len() > 0 { Ok(Some(filtered_items[selected].0)) }
        else { Ok(None) }
    }

    fn filter_items(&self, filter: &[char]) -> Vec<(usize, &String)> {
        let mut filtered: Vec<(usize, &String, f32)>  = self.items
            .iter()
            .enumerate()
            .filter_map(|(index, item)| {
                let (matches, score) = self.fuzzy_match(item, filter);
                if matches {
                    Some((index, item, score))
                } else {
                    None
                }
            })
            .collect();

        filtered.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
        filtered.into_iter().map(|(index, item, _)| (index, item)).collect()
    }

    fn fuzzy_match(&self, item: &str, filter: &[char]) -> (bool, f32) {
        let item_lower: Vec<char> = item.to_lowercase().chars().collect(); 

        let mut item_index = 0;
        let mut mistakes = 0.0;
        let max_mistakes: f32 = filter.len() as f32 * self.max_mistake_ratio;

        for &filter_char in filter{
            if let Some(pos) = item_lower[item_index..].iter().position(|&c| c == filter_char.to_ascii_lowercase()) {
                item_index += pos + 1;
                mistakes += pos as f32 * 0.1;
            } else {
                mistakes += 1.0;
            }
            if mistakes > max_mistakes {
                return (false, 0.0);
            }
        }
        let match_quality = 1.0 - (mistakes/max_mistakes);
        (true, match_quality)
    }
}
