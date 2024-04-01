use eframe::egui::{Button, Color32, CursorIcon, Response, RichText, Ui};
use std::fs;
use std::io::Result;
use std::path::Path;

use super::ui::{SharedActiveFilePath, FILE_TREE_WIDTH};

const BUTTON_CURSOR: CursorIcon = CursorIcon::PointingHand;

pub struct FileTree {
    pub active_file_path: SharedActiveFilePath,
    pub relative_path: Vec<String>,
}

impl FileTree {
    pub fn new(active_file_path: SharedActiveFilePath) -> Self {
        FileTree {
            active_file_path,
            relative_path: vec![],
        }
    }

    fn is_dir<P: AsRef<Path>>(path: P) -> Result<bool> {
        let metadata = fs::metadata(path)?;
        Ok(metadata.is_dir())
    }

    fn add_back_button(&mut self, ui: &mut Ui) {
        let go_back_button = FileTreeButton::build(ui, &String::from(".."), false, false);

        if go_back_button.clicked() {
            if self.relative_path.len() > 0 {
                if self.relative_path.last().unwrap() != ".." {
                    self.relative_path.pop();
                } else {
                    self.relative_path.push("..".to_string());
                }
            } else {
                self.relative_path.push("..".to_string());
            }
        }
    }

    fn file_list(&mut self) -> Vec<String> {
        let mut target_dir = std::env::current_dir().expect("Unable to get current dir");

        for dir in &self.relative_path {
            if dir == ".." {
                target_dir.pop();
            } else {
                target_dir.push(dir);
            }
        }

        let mut result = Vec::new();

        let path = fs::read_dir(target_dir).unwrap();
        for entry in path {
            let file_path = entry.expect("Failed to get directory entry").path();
            let file_path_str = file_path.to_string_lossy().into_owned();

            result.push(file_path_str);
        }

        result
    }

    pub fn draw(&mut self, ui: &mut Ui) {
        self.add_back_button(ui);

        for file_path in self.file_list() {
            let entry_is_dir = Self::is_dir(&file_path).unwrap();

            let is_active = match self.active_file_path.borrow().as_ref() {
                Some(path) => *path == file_path,
                None => false,
            };

            let ui_element = FileTreeButton::build(ui, &file_path, entry_is_dir, is_active);
            if ui_element.clicked() {
                if entry_is_dir {
                    self.relative_path.push(file_path.to_string());
                } else {
                    self.active_file_path.replace(Some(file_path));
                }
            }
        }
    }
}

struct FileTreeButton {}
impl FileTreeButton {
    fn path_to_file_name(path: &String) -> String {
        path.split(std::path::MAIN_SEPARATOR)
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .to_string()
    }

    fn build(ui: &mut Ui, file_path: &String, is_directory: bool, is_active: bool) -> Response {
        let file_name = Self::path_to_file_name(&file_path);

        let file_name_text = if is_directory {
            format!("[ ] {}", file_name)
        } else {
            format!("{}", file_name)
        };

        let (color_bg, color_fg) = if is_active {
            (Color32::from_rgb(255, 255, 255), Color32::from_rgb(0, 0, 0))
        } else {
            (Color32::from_rgb(0, 0, 0), Color32::from_rgb(255, 255, 255))
        };

        let button_text = RichText::new(file_name_text).color(color_fg);
        let button = Button::new(button_text).fill(color_bg);
        ui.add_sized([FILE_TREE_WIDTH, 20.], button)
            .on_hover_and_drag_cursor(BUTTON_CURSOR)
    }
}
