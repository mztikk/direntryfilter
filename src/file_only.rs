use crate::IgnoreDirEntry;

/// A filter that matches only files.
#[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FileOnlyFilter;

impl FileOnlyFilter {
    /// Creates a new file only filter
    pub fn new() -> Self {
        Self::default()
    }
}

impl IgnoreDirEntry for FileOnlyFilter {
    fn ignore(&self, entry: &std::fs::DirEntry) -> bool {
        match entry.file_type() {
            Ok(file_type) => file_type.is_dir() || file_type.is_symlink(),
            Err(_) => false,
        }
    }
}
