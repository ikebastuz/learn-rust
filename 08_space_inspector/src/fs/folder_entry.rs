use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub enum FolderEntryType {
    Parent,
    File,
    Folder,
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
