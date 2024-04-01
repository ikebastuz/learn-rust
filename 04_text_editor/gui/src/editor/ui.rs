use crate::editor::content::Content;
use crate::editor::file_tree::FileTree;
use eframe::egui::{menu, CentralPanel, Context, Window};
use std::cell::RefCell;
use std::rc::Rc;

pub type SharedActiveFilePath = Rc<RefCell<Option<String>>>;
pub type SharedNotification = Rc<RefCell<Option<String>>>;

pub const FILE_TREE_WIDTH: f32 = 200.;

pub struct EditorApp {
    notification: SharedNotification,
    file_tree: FileTree,
    content: Content,
}

impl Default for EditorApp {
    fn default() -> Self {
        let active_file_path = Rc::new(RefCell::new(None));
        let notification = Rc::new(RefCell::new(None));
        Self {
            notification: Rc::clone(&notification),
            file_tree: FileTree::new(Rc::clone(&active_file_path)),
            content: Content::new(Rc::clone(&active_file_path), Rc::clone(&notification)),
        }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let mut width = 0.0;
        ctx.input(|i| {
            if let Some(inner_rect) = i.viewport().inner_rect {
                width = inner_rect.width();
            }
        });

        CentralPanel::default().show(ctx, move |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Save").clicked() {
                        self.content.save();
                        self.notification.replace(Some(String::from("Saved")));
                        ui.close_menu();
                    }
                    if ui.button("Exit").clicked() {
                        std::process::exit(0);
                    }
                });
            });

            ui.horizontal(|ui| {
                ui.columns(2, |cols| {
                    cols[0].vertical(|ui| {
                        ui.set_width(FILE_TREE_WIDTH);
                        ui.label("Files:");
                        self.file_tree.draw(ui);
                    });
                    cols[1].vertical_centered_justified(|ui| {
                        ui.set_width(width - FILE_TREE_WIDTH);
                        ui.label("Document:");
                        self.content.draw(ui);
                    });
                });
            });
            ui.horizontal(|ui| {
                match self.notification.borrow().as_ref() {
                    Some(message) => {
                        ui.label(message);
                    }
                    None => {
                        ui.label("");
                    }
                };
            });
        });
    }
}
