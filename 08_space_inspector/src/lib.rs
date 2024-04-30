use std::collections::HashMap;
use std::env::current_dir;
use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::prelude::*;

mod fs;
mod ui;

use fs::{path_to_folder, Folder};

#[derive(Debug)]
pub struct App {
    file_tree_map: HashMap<String, Folder>,
}

impl App {
    pub fn new() -> Self {
        let cd = current_dir().unwrap();
        let initial_folder = path_to_folder(&cd);

        let mut file_tree_map = HashMap::new();
        file_tree_map.insert(cd.to_string_lossy().into_owned(), initial_folder);

        App { file_tree_map }
    }

    fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
        terminal.draw(|f| f.render_widget(self, f.size()))?;
        Ok(())
    }

    pub fn run(&mut self, mut terminal: Terminal<impl Backend>) -> io::Result<()> {
        loop {
            self.draw(&mut terminal)?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    use KeyCode::*;
                    match key.code {
                        Char('q') | Esc => return Ok(()),
                        Char('j') | Down => self.todo(),
                        Char('k') | Up => self.todo(),
                        _ => {}
                    }
                }
            }
        }
    }

    fn todo(&mut self) {
        println!("TODO:");
    }
}
