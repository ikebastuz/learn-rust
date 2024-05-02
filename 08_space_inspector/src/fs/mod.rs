use std::cmp::Ordering;
use std::fs::{read_dir, remove_dir_all, remove_file};
use std::path::PathBuf;

use crate::ui::{TEXT_PARENT_DIR, TEXT_UNKNOWN};

#[derive(Debug, Clone, PartialOrd, Eq, PartialEq)]
pub struct FolderEntry {
    pub title: String,
    pub size: Option<u64>,
}

impl Ord for FolderEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.title.cmp(&other.title)
    }
}

#[derive(Debug, Clone)]
pub struct Folder {
    pub title: String,
    pub total_size: u64,
    pub cursor_index: usize,
    pub files: Vec<FolderEntry>,
    pub folders: Vec<FolderEntry>,
}

impl Folder {
    pub fn new(title: String) -> Self {
        Folder {
            title,
            total_size: 0,
            cursor_index: 0,
            files: Vec::new(),
            folders: Vec::new(),
        }
    }

    pub fn to_list(&self) -> Vec<FolderEntry> {
        vec![
            &vec![FolderEntry {
                title: String::from(TEXT_PARENT_DIR),
                size: None,
            }],
            &self.folders,
            &self.files,
        ]
        .into_iter()
        .flat_map(|v| v.iter().cloned())
        .collect()
    }

    pub fn get_max_entry_size(&self) -> u64 {
        let mut max_entry_size = 0;

        for file in &self.files {
            if let Some(size) = file.size {
                if size > max_entry_size {
                    max_entry_size = size
                }
            }
        }
        for folder in &self.folders {
            if let Some(size) = folder.size {
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
                    title: file_name.to_owned(),
                    size: None,
                };
                if entry.path().is_dir() {
                    folder.folders.push(folder_entry);
                } else {
                    let metadata = entry.metadata().expect("Failed to get metadata");
                    let size = metadata.len();
                    folder.total_size += size;

                    folder_entry.size = Some(size);
                    folder.files.push(folder_entry);
                }
            }
        }
    }

    folder.files.sort();
    folder.folders.sort();

    folder
}

pub fn delete_folder(path: &PathBuf) -> Result<(), std::io::Error> {
    remove_dir_all(path)
}

pub fn delete_file(path: &PathBuf) -> Result<(), std::io::Error> {
    remove_file(path)
}