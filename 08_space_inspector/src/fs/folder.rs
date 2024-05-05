use crate::ui::TEXT_PARENT_DIR;

use crate::fs::folder_entry::{FolderEntry, FolderEntryType};

#[derive(Debug, Clone)]
pub struct Folder {
    pub title: String,
    pub cursor_index: usize,
    pub entries: Vec<FolderEntry>,
}

impl Folder {
    pub fn new(title: String) -> Self {
        Folder {
            title,
            cursor_index: 0,
            entries: vec![FolderEntry {
                kind: FolderEntryType::Parent,
                title: String::from(TEXT_PARENT_DIR),
                size: None,
            }],
        }
    }

    pub fn get_size(&self) -> u64 {
        self.entries
            .iter()
            .fold(0, |acc, entry| acc + entry.size.unwrap_or(0))
    }

    pub fn get_selected_entry_size(&self) -> u64 {
        self.get_selected_entry().size.unwrap_or(0)
    }

    pub fn remove_selected(&mut self) {
        self.entries.remove(self.cursor_index);
        if self.cursor_index > self.entries.len() - 1 {
            self.cursor_index = self.entries.len() - 1
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
