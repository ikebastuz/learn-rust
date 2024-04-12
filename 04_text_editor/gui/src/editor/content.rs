use eframe::egui::{ScrollArea, TextEdit, TextStyle, Ui};
use egui_extras::syntax_highlighting;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use super::ui::{SharedActiveFilePath, SharedNotification};

pub struct Content {
    pub file_contents: HashMap<String, String>,
    pub active_file_path: SharedActiveFilePath,
    pub notification: SharedNotification,
}

impl Content {
    pub fn new(active_file_path: SharedActiveFilePath, notification: SharedNotification) -> Self {
        Content {
            file_contents: HashMap::new(),
            active_file_path,
            notification,
        }
    }

    pub fn draw(&mut self, ui: &mut Ui) {
        let active_file_path = self.active_file_path.borrow();

        match &*active_file_path {
            Some(fp) => {
                let fp_str = fp.as_str();
                if let Some(content) = self.file_contents.get_mut(fp_str) {
                    let language = Path::new(fp_str)
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .unwrap_or("txt");

                    let theme = syntax_highlighting::CodeTheme::from_memory(ui.ctx());

                    let mut layouter = |ui: &Ui, string: &str, wrap_width: f32| {
                        let mut layout_job =
                            syntax_highlighting::highlight(ui.ctx(), &theme, string, language);
                        layout_job.wrap.max_width = wrap_width;
                        ui.fonts(|f| f.layout_job(layout_job))
                    };

                    ScrollArea::vertical().show(ui, |ui| {
                        ui.add(
                            TextEdit::multiline(content)
                                .font(TextStyle::Monospace)
                                .code_editor()
                                .desired_rows(10)
                                .lock_focus(true)
                                .desired_width(f32::INFINITY)
                                .layouter(&mut layouter),
                        );
                    });
                } else {
                    let contents = fs::read_to_string(fp_str).unwrap_or_else(|err| {
                        if err.kind() == std::io::ErrorKind::NotFound {
                            self.notification
                                .replace(Some(String::from(format!("File not found: {}", err))));
                        } else {
                            self.notification.replace(Some(String::from(format!(
                                "Error reading file: {}",
                                err
                            ))));
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
                    self.notification.replace(Some(String::from("Saved!")));
                } else {
                    self.notification
                        .replace(Some(String::from("No content to save")));
                }
            }
            _ => {
                self.notification
                    .replace(Some(String::from("No file selected")));
            }
        }
    }
}
