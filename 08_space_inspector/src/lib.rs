use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::read_dir;
use std::io;
use std::path::PathBuf;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

#[derive(Debug, Clone)]
pub struct FileTreeNode<'a> {
    pub name: String,
    pub files: Vec<String>,
    pub folders: Vec<FileTreeNode<'a>>,
    pub parent: Option<&'a FileTreeNode<'a>>,
}

impl<'a> FileTreeNode<'a> {
    pub fn new(name: &str, parent: Option<&'a FileTreeNode<'a>>) -> Self {
        FileTreeNode {
            name: String::from(name),
            files: Vec::new(),
            folders: Vec::new(),
            parent,
        }
    }

    pub fn add_file(&mut self, filename: &str) {
        self.files.push(String::from(filename));
    }

    pub fn add_folder(&mut self, folder: FileTreeNode<'a>) {
        self.folders.push(folder);
    }
}

#[derive(Debug)]
pub struct Folder {
    files: Vec<String>,
    folders: Vec<String>,
}

impl Folder {
    fn new() -> Self {
        Folder {
            files: Vec::new(),
            folders: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct App {
    file_tree_map: HashMap<String, Folder>,
}

impl App {
    pub fn new() -> Self {
        let cd = current_dir().unwrap();
        let initial_folder = App::path_to_folder(&cd);

        let mut file_tree_map = HashMap::new();
        file_tree_map.insert(cd.to_string_lossy().into_owned(), initial_folder);

        App { file_tree_map }
    }

    fn path_to_folder(path: &PathBuf) -> Folder {
        let mut folder = Folder::new();

        let path = read_dir(path.clone()).expect("Failed to read directory");
        for entry in path {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                if let Some(file_name) = file_name.to_str() {
                    if entry.path().is_dir() {
                        folder.folders.push(file_name.to_owned());
                    } else {
                        folder.files.push(file_name.to_owned());
                    }
                }
            }
        }

        folder
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
        todo!();
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(2),
        ]);
        let [header_area, rest_area, footer_area] = vertical.areas(area);

        render_title(header_area, buf);
        render_footer(footer_area, buf);
    }
}

fn render_title(area: Rect, buf: &mut Buffer) {
    Paragraph::new("Ratatui List Example")
        .bold()
        .centered()
        .render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Paragraph::new("\nUse ↓↑ to move, ← to unselect, → to change status, g/G to go top/bottom.")
        .centered()
        .render(area, buf);
}
