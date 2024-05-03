use std::io;
use std::path::Path;
use std::{collections::HashMap, path::PathBuf};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::prelude::*;
use std::env;

mod fs;
mod ui;

use fs::{delete_file, delete_folder, process_filepath, Folder, FolderEntryType};

#[derive(Debug)]
pub struct App {
    current_path: String,
    file_tree_map: HashMap<String, Folder>,
}

impl App {
    pub fn new(file_path: Option<String>) -> Self {
        let current_path = match file_path {
            Some(path) => {
                let path_buf = PathBuf::from(&path);
                if path_buf.is_absolute() {
                    path_buf.to_string_lossy().into_owned()
                } else {
                    let current_dir = env::current_dir().unwrap();
                    let abs_path = current_dir.join(&path_buf);
                    abs_path.to_string_lossy().into_owned()
                }
            }
            None => env::current_dir().unwrap().to_string_lossy().into_owned(),
        };

        let mut file_tree_map = HashMap::new();
        process_filepath(&mut file_tree_map, &PathBuf::from(&current_path));

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
                        Char('j') | Down => self.cursor_down(),
                        Char('k') | Up => self.cursor_up(),
                        Char('d') | Delete => self.delete_pressed(),
                        Enter => self.enter_pressed(),
                        _ => {}
                    }
                }
            }
        }
    }

    fn cursor_up(&mut self) {
        if let Some(mut folder) = self.get_current_folder().cloned() {
            if folder.cursor_index > 0 {
                folder.cursor_index -= 1;
                self.file_tree_map.insert(self.current_path.clone(), folder);
            }
        }
    }

    fn cursor_down(&mut self) {
        if let Some(mut folder) = self.get_current_folder().cloned() {
            if folder.cursor_index < folder.entries.len() - 1 {
                folder.cursor_index += 1;
                self.file_tree_map.insert(self.current_path.clone(), folder);
            }
        }
    }

    fn get_current_folder(&self) -> Option<&Folder> {
        self.file_tree_map.get(&self.current_path)
    }

    fn propagate_size_update_upwards(&mut self, to_delete_path: &PathBuf, deleted_entry_size: u64) {
        let mut parent_path = to_delete_path.clone();
        while let Some(parent) = parent_path.parent() {
            if let Some(parent_folder) = self.file_tree_map.get_mut(parent.to_str().unwrap()) {
                if let Some(parent_folder_entry) =
                    parent_folder.entries.get_mut(parent_folder.cursor_index)
                {
                    if let Some(size) = parent_folder_entry.size.as_mut() {
                        *size -= deleted_entry_size;
                    }
                }
                parent_path = parent.to_path_buf();
            } else {
                break;
            }
        }
    }

    fn delete_pressed(&mut self) {
        if let Some(mut folder) = self.get_current_folder().cloned() {
            let entry = folder.get_selected_entry();

            let mut to_delete_path = PathBuf::from(&self.current_path);
            to_delete_path.push(&entry.title);

            match entry.kind {
                FolderEntryType::Parent => {}
                FolderEntryType::Folder => {
                    if let Ok(_) = delete_folder(&to_delete_path) {
                        if let Some(subfolder_size) = entry.size {
                            self.propagate_size_update_upwards(&to_delete_path, subfolder_size);
                        }
                        folder.remove_selected();
                        let path_string = to_delete_path.to_string_lossy().into_owned();
                        self.file_tree_map.remove(&path_string);
                        self.file_tree_map.insert(self.current_path.clone(), folder);
                    }
                }
                FolderEntryType::File => {
                    if let Ok(_) = delete_file(&to_delete_path) {
                        if let Some(subfile_size) = entry.size {
                            self.propagate_size_update_upwards(&to_delete_path, subfile_size);
                        }
                        folder.remove_selected();
                        self.file_tree_map.insert(self.current_path.clone(), folder);
                    }
                }
            }
        }
    }

    fn enter_pressed(&mut self) {
        if let Some(folder) = self.get_current_folder().cloned() {
            let entry = folder.get_selected_entry();

            match entry.kind {
                FolderEntryType::Parent => {
                    if let Some(parent) = Path::new(&self.current_path).parent() {
                        if let Some(parent_path) = parent.to_str() {
                            self.current_path = parent_path.to_owned();
                            self.process_filepath_if_not_exist();
                            return;
                        }
                    }
                }
                FolderEntryType::Folder => {
                    let mut new_path = PathBuf::from(&self.current_path);
                    new_path.push(&entry.title);
                    self.current_path = new_path.to_string_lossy().into_owned();
                    self.process_filepath_if_not_exist();
                    return;
                }
                FolderEntryType::File => {
                    return;
                }
            }
        }
    }

    fn process_filepath_if_not_exist(&mut self) {
        if !self.file_tree_map.contains_key(&self.current_path) {
            process_filepath(&mut self.file_tree_map, &PathBuf::from(&self.current_path));
        }
    }
}

#[path = "tests.rs"]
mod tests;
