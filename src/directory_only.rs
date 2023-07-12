use crate::IgnoreDirEntry;

/// A filter that matches only directories.
#[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DirectoryOnlyFilter;

impl DirectoryOnlyFilter {
    /// Creates a new directory only filter
    pub fn new() -> Self {
        Self::default()
    }
}

impl IgnoreDirEntry for DirectoryOnlyFilter {
    fn ignore(&self, entry: &std::fs::DirEntry) -> bool {
        match entry.file_type() {
            Ok(file_type) => file_type.is_file() || file_type.is_symlink(),
            Err(_) => false,
        }
    }
}
