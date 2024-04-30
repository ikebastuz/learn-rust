use std::fs::read_dir;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Folder {
    pub files: Vec<String>,
    pub folders: Vec<String>,
}

impl Folder {
    pub fn new() -> Self {
        Folder {
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
