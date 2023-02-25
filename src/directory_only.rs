use crate::DirEntryFilter;

/// A filter that matches only directories.
#[derive(Default)]
pub struct DirectoryOnlyFilter;

impl DirectoryOnlyFilter {
    /// Creates a new directory only filter
    pub fn new() -> Self {
        DirectoryOnlyFilter::default()
    }
}

impl DirEntryFilter for DirectoryOnlyFilter {
    fn ignore(&self, entry: &std::fs::DirEntry) -> bool {
        match entry.file_type() {
            Ok(file_type) => file_type.is_file() || file_type.is_symlink(),
            Err(_) => false,
        }
    }
}
