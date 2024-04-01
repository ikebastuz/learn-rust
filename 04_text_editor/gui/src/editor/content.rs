use eframe::egui::Ui;
use std::collections::HashMap;
use std::fs;

use super::ui::SharedActiveFilePath;

pub struct Content {
    pub file_contents: HashMap<String, String>,
    pub active_file_path: SharedActiveFilePath,
}

impl Content {
    pub fn new(active_file_path: SharedActiveFilePath) -> Self {
        Content {
            file_contents: HashMap::new(),
            active_file_path,
        }
    }

    pub fn draw(&mut self, ui: &mut Ui) {
        let active_file_path = self.active_file_path.borrow();

        match &*active_file_path {
            Some(fp) => {
                let fp_str = fp.as_str();
                if let Some(content) = self.file_contents.get_mut(fp_str) {
                    ui.text_edit_multiline(content);
                } else {
                    let contents = fs::read_to_string(fp_str).unwrap_or_else(|err| {
                        if err.kind() == std::io::ErrorKind::NotFound {
                            eprintln!("File not found: {}", err);
                        } else {
                            eprintln!("Error reading file: {}", err);
                        }
                        "".to_string()
                    });
                    self.file_contents.insert(fp_str.to_string(), contents);
                }
            }
            _ => {}
        }
    }

    pub fn save(&self) {
        let active_file_path = self.active_file_path.borrow();

        match &*active_file_path {
            Some(fp) => {
                let fp_str = fp.as_str();
                if let Some(content) = self.file_contents.get(fp_str) {
                    let _ = fs::write(fp_str, content);
                } else {
                    eprintln!("No content to save");
                }
            }
            _ => {
                eprintln!("No file selected");
            }
        }
    }
}
