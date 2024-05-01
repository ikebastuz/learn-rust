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
    current_path: String,
    file_tree_map: HashMap<String, Folder>,
}

impl App {
    pub fn new() -> Self {
        let cd = current_dir().unwrap();
        let initial_folder = path_to_folder(&cd);

        let mut file_tree_map = HashMap::new();
        let current_path = cd.to_string_lossy().into_owned();
        file_tree_map.insert(current_path.clone(), initial_folder);

        App {
            file_tree_map,
            current_path,
        }
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
                        Char('j') | Down => self.todo_down(),
                        Char('k') | Up => self.todo_up(),
                        _ => {}
                    }
                }
            }
        }
    }

    fn todo_up(&mut self) {
        if let Some(mut folder) = self.get_current_dir_list().cloned() {
            if folder.cursor_index > 0 {
                folder.cursor_index -= 1;
                self.file_tree_map.insert(self.current_path.clone(), folder);
            }
        }
    }

    fn todo_down(&mut self) {
        if let Some(mut folder) = self.get_current_dir_list().cloned() {
            if folder.cursor_index < folder.files.len() + folder.folders.len() {
                folder.cursor_index += 1;
                self.file_tree_map.insert(self.current_path.clone(), folder);
            }
        }
    }

    fn get_current_dir_list(&self) -> Option<&Folder> {
        self.file_tree_map.get(&self.current_path)
    }
}
