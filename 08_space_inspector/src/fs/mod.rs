use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::{read_dir, remove_dir_all, remove_file};
use std::path::PathBuf;

use crate::ui::{TEXT_PARENT_DIR, TEXT_UNKNOWN};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub enum FolderEntryType {
    Parent,
    File,
    Folder,
    Unknown,
}

impl Ord for FolderEntryType {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (FolderEntryType::Parent, _) => Ordering::Less,
            (FolderEntryType::Folder, FolderEntryType::Parent) => Ordering::Greater,
            (FolderEntryType::Folder, FolderEntryType::Folder) => Ordering::Equal,
            (FolderEntryType::Folder, _) => Ordering::Less,
            (FolderEntryType::File, FolderEntryType::Parent) => Ordering::Greater,
            (FolderEntryType::File, FolderEntryType::Folder) => Ordering::Greater,
            (FolderEntryType::File, FolderEntryType::File) => Ordering::Equal,
            (FolderEntryType::File, _) => Ordering::Less,
            (FolderEntryType::Unknown, _) => Ordering::Greater,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FolderEntry {
    pub title: String,
    pub size: Option<u64>,
    pub kind: FolderEntryType,
}

impl Ord for FolderEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        let kind_ordering = self.kind.cmp(&other.kind);

        if kind_ordering != Ordering::Equal {
            return kind_ordering;
        }

        self.title.cmp(&other.title)
    }
}

impl PartialOrd for FolderEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
pub struct Folder {
    pub title: String,
    pub total_size: u64,
    pub cursor_index: usize,
    pub entries: Vec<FolderEntry>,
}

impl Folder {
    pub fn new(title: String) -> Self {
        Folder {
            title,
            total_size: 0,
            cursor_index: 0,
            entries: vec![FolderEntry {
                kind: FolderEntryType::Parent,
                title: String::from(TEXT_PARENT_DIR),
                size: None,
            }],
        }
    }

    pub fn get_selected_entry_size(&self) -> u64 {
        self.get_selected_entry().size.unwrap_or(0)
    }

    pub fn remove_selected_folder(&mut self) {
        let entry = self.get_selected_entry();
        if entry.kind == FolderEntryType::Folder {
            self.entries.remove(self.cursor_index);
        }
    }

    pub fn remove_selected_file(&mut self) {
        let entry = self.get_selected_entry();
        if entry.kind == FolderEntryType::File {
            self.entries.remove(self.cursor_index);
        }
    }

    pub fn get_selected_entry(&self) -> &FolderEntry {
        if let Some(entry) = self.entries.get(self.cursor_index) {
            entry
        } else {
            panic!("Cursor index out of bounds: {}", self.cursor_index);
        }
    }

    pub fn to_list(&self) -> Vec<FolderEntry> {
        vec![&self.entries]
            .into_iter()
            .flat_map(|v| v.iter().cloned())
            .collect()
    }

    pub fn get_max_entry_size(&self) -> u64 {
        let mut max_entry_size = 0;

        for file in &self.entries {
            if let Some(size) = file.size {
                if size > max_entry_size {
                    max_entry_size = size
                }
            }
        }

        max_entry_size
    }
}

pub fn path_to_folder(path: &PathBuf) -> Folder {
    let folder_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(TEXT_UNKNOWN);
    let mut folder = Folder::new(folder_name.to_string());

    let path = read_dir(path.clone()).expect("Failed to read directory");
    for entry in path {
        if let Ok(entry) = entry {
            let file_name = entry.file_name();
            if let Some(file_name) = file_name.to_str() {
                let mut folder_entry = FolderEntry {
                    kind: FolderEntryType::File,
                    title: file_name.to_owned(),
                    size: None,
                };
                if entry.path().is_dir() {
                    folder_entry.kind = FolderEntryType::Folder;
                    folder.entries.push(folder_entry);
                } else {
                    let metadata = entry.metadata().expect("Failed to get metadata");
                    let size = metadata.len();
                    folder.total_size += size;

                    folder_entry.size = Some(size);
                    folder.entries.push(folder_entry);
                }
            }
        }
    }

    folder.entries.sort();

    folder
}

pub fn delete_folder(path: &PathBuf) -> Result<(), std::io::Error> {
    remove_dir_all(path)
}

pub fn delete_file(path: &PathBuf) -> Result<(), std::io::Error> {
    remove_file(path)
}

pub fn process_filepath(file_tree: &mut HashMap<String, Folder>, path_buf: &PathBuf) -> u64 {
    let path_string = path_buf.to_string_lossy().into_owned();

    if let Some(folder) = file_tree.get(&path_string) {
        return folder.total_size;
    }

    let mut folder = path_to_folder(path_buf);

    for child_entry in folder.entries.iter_mut() {
        if child_entry.kind == FolderEntryType::Folder {
            let mut subfolder_path = path_buf.clone();
            subfolder_path.push(&child_entry.title);

            let subfolder_size = process_filepath(file_tree, &subfolder_path);
            child_entry.size = Some(subfolder_size);
            folder.total_size += subfolder_size;
        }
    }

    let total_size = folder.total_size.clone();

    file_tree.insert(path_string, folder);

    total_size
}
