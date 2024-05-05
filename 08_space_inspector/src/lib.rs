use opener;
use std::io;
use std::{collections::HashMap, path::PathBuf};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::prelude::*;
use std::env;

mod fs;
mod ui;

use fs::{delete_file, delete_folder, path_to_folder, Folder, FolderEntryType, SortBy};
use ui::UIConfig;

#[derive(Debug)]
pub struct App {
    current_path: PathBuf,
    file_tree_map: HashMap<String, Folder>,
    ui_config: UIConfig,
}

enum DiffKind {
    Subtract,
}

impl App {
    pub fn new() -> Self {
        App {
            file_tree_map: HashMap::new(),
            current_path: PathBuf::from("."),
            ui_config: UIConfig {
                colored: false,
                confirming_deletion: false,
                sort_by: SortBy::Title,
                move_to_trash: true,
                open_file: true,
            },
        }
    }

    pub fn init(&mut self, file_path: Option<String>) {
        let current_path = match file_path {
            Some(path) => {
                let path_buf = PathBuf::from(&path);
                if path_buf.is_absolute() {
                    path_buf
                } else {
                    let current_dir = env::current_dir().unwrap();
                    let abs_path = current_dir.join(&path_buf);
                    abs_path
                }
            }
            None => env::current_dir().unwrap(),
        };

        self.current_path = current_path;
        self.process_filepath(&self.current_path.clone());
    }

    fn get_current_path_string(&self) -> String {
        self.current_path.to_string_lossy().to_string()
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
                        Char('j') | Down => self.on_cursor_down(),
                        Char('k') | Up => self.on_cursor_up(),
                        Char('d') | Delete => self.on_delete(),
                        Char('s') => self.on_toggle_sorting(),
                        Char('c') => self.on_toggle_coloring(),
                        Char('t') => self.on_toggle_move_to_trash(),
                        Backspace => self.on_backspace(),
                        Enter => self.on_enter(),
                        _ => {}
                    }
                }
            }
        }
    }

    fn on_toggle_move_to_trash(&mut self) {
        self.ui_config.move_to_trash = !self.ui_config.move_to_trash;
    }

    fn on_toggle_coloring(&mut self) {
        self.ui_config.colored = !self.ui_config.colored;
    }

    fn on_toggle_sorting(&mut self) {
        match self.ui_config.sort_by {
            SortBy::Title => {
                self.ui_config.sort_by = SortBy::Size;
            }
            SortBy::Size => {
                self.ui_config.sort_by = SortBy::Title;
            }
        }

        self.sort_current_folder();
    }

    fn on_backspace(&mut self) {
        self.navigate_to_parent();
    }

    fn sort_current_folder(&mut self) {
        if let Some(mut folder) = self.get_current_folder().cloned() {
            match self.ui_config.sort_by {
                SortBy::Size => {
                    folder.sort_by_size();
                }
                SortBy::Title => {
                    folder.sort_by_title();
                }
            }
            self.set_current_folder(folder);
        }
    }

    fn get_current_folder_v2(&mut self) -> Option<&mut Folder> {
        self.file_tree_map.get_mut(&self.get_current_path_string())
    }

    fn on_cursor_up(&mut self) {
        if let Some(folder) = self.get_current_folder_v2() {
            if folder.cursor_index > 0 {
                folder.cursor_index -= 1;
            }
        }
        self.ui_config.confirming_deletion = false;
    }

    fn on_cursor_down(&mut self) {
        if let Some(folder) = self.get_current_folder_v2() {
            if folder.cursor_index < folder.entries.len() - 1 {
                folder.cursor_index += 1;
            }
        }
        self.ui_config.confirming_deletion = false;
    }

    fn get_current_folder(&self) -> Option<&Folder> {
        self.file_tree_map.get(&self.get_current_path_string())
    }

    fn set_current_folder(&mut self, folder: Folder) {
        self.file_tree_map
            .insert(self.get_current_path_string(), folder);
    }

    fn propagate_size_update_upwards(
        &mut self,
        to_delete_path: &PathBuf,
        entry_diff: u64,
        diff_kind: DiffKind,
    ) {
        let mut parent_path = to_delete_path.clone();
        while let Some(parent) = parent_path.parent() {
            if let Some(parent_folder) = self.file_tree_map.get_mut(parent.to_str().unwrap()) {
                if let Some(parent_folder_entry) =
                    parent_folder.entries.get_mut(parent_folder.cursor_index)
                {
                    if let Some(size) = parent_folder_entry.size.as_mut() {
                        match diff_kind {
                            DiffKind::Subtract => *size -= entry_diff,
                        }
                    }
                }
                parent_path = parent.to_path_buf();
            } else {
                break;
            }
        }
    }

    fn on_delete(&mut self) {
        if let Some(mut folder) = self.get_current_folder().cloned() {
            let entry = folder.get_selected_entry();

            let mut to_delete_path = PathBuf::from(&self.current_path);
            to_delete_path.push(&entry.title);

            match entry.kind {
                FolderEntryType::Parent => {}
                FolderEntryType::Folder => {
                    if !self.ui_config.confirming_deletion {
                        self.ui_config.confirming_deletion = true;
                    } else {
                        if let Ok(_) = delete_folder(&to_delete_path, &self.ui_config) {
                            if let Some(subfolder_size) = entry.size {
                                self.propagate_size_update_upwards(
                                    &to_delete_path,
                                    subfolder_size,
                                    DiffKind::Subtract,
                                );
                            }
                            folder.remove_selected();
                            let path_string = to_delete_path.to_string_lossy().into_owned();
                            self.file_tree_map.remove(&path_string);
                            self.set_current_folder(folder);
                            self.ui_config.confirming_deletion = false;
                        }
                    }
                }
                FolderEntryType::File => {
                    if !self.ui_config.confirming_deletion {
                        self.ui_config.confirming_deletion = true;
                    } else {
                        if let Ok(_) = delete_file(&to_delete_path, &self.ui_config) {
                            if let Some(subfile_size) = entry.size {
                                self.propagate_size_update_upwards(
                                    &to_delete_path,
                                    subfile_size,
                                    DiffKind::Subtract,
                                );
                            }
                            folder.remove_selected();
                            self.set_current_folder(folder);
                            self.ui_config.confirming_deletion = false;
                        }
                    }
                }
            }
        }
    }

    fn navigate_to_parent(&mut self) {
        if let Some(parent) = PathBuf::from(&self.current_path).parent() {
            let parent_buf = parent.to_path_buf();
            self.current_path = parent_buf.clone();
            self.process_filepath(&parent_buf);
            self.sort_current_folder();
        }
    }

    fn navigate_to_child(&mut self, title: &String) {
        let mut new_path = PathBuf::from(&self.current_path);
        new_path.push(title);
        self.current_path = new_path;
        self.process_filepath(&PathBuf::from(&self.current_path));
        self.sort_current_folder();
    }

    fn on_enter(&mut self) {
        if let Some(folder) = self.get_current_folder().cloned() {
            let entry = folder.get_selected_entry();

            match entry.kind {
                FolderEntryType::Parent => {
                    self.navigate_to_parent();
                }
                FolderEntryType::Folder => {
                    self.navigate_to_child(&entry.title);
                }
                FolderEntryType::File => {
                    if self.ui_config.open_file {
                        let mut file_name = PathBuf::from(&self.current_path.clone());
                        file_name.push(entry.title.clone());
                        let _ = opener::open(file_name);
                    }
                }
            }
        }
        self.ui_config.confirming_deletion = false;
    }

    fn process_filepath(&mut self, path_buf: &PathBuf) -> u64 {
        if !self
            .file_tree_map
            .contains_key(&path_buf.to_string_lossy().to_string())
        {
            let path_string = path_buf.to_string_lossy().into_owned();

            if let Some(folder) = self.file_tree_map.get(&path_string) {
                return folder.get_size();
            }

            let mut folder = path_to_folder(path_buf);

            for child_entry in folder.entries.iter_mut() {
                if child_entry.kind == FolderEntryType::Folder {
                    let mut subfolder_path = path_buf.clone();
                    subfolder_path.push(&child_entry.title);

                    let subfolder_size = self.process_filepath(&subfolder_path);
                    child_entry.size = Some(subfolder_size);
                }
            }

            let total_size = folder.get_size();

            self.file_tree_map.insert(path_string, folder);

            return total_size;
        }

        0
    }
}

#[path = "tests.rs"]
mod tests;
