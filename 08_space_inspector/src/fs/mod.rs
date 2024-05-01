use std::cmp::Ordering;
use std::fs::read_dir;
use std::path::PathBuf;

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
    pub cursor_index: usize,
    pub files: Vec<FolderEntry>,
    pub folders: Vec<FolderEntry>,
}

impl Folder {
    pub fn new() -> Self {
        Folder {
            cursor_index: 0,
            files: Vec::new(),
            folders: Vec::new(),
        }
    }
}

pub fn path_to_folder(path: &PathBuf) -> Folder {
    let mut folder = Folder::new();

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

                    folder_entry.size = Some(size);
                    println!("Size: {}", size);
                    folder.files.push(folder_entry);
                }
            }
        }
    }

    folder.files.sort();
    folder.folders.sort();

    folder
}
