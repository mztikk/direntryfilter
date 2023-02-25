use crate::DirEntryFilter;

/// A filter that matches only files.
#[derive(Default)]
pub struct FileOnlyFilter;

impl FileOnlyFilter {
    /// Creates a new file only filter
    pub fn new() -> Self {
        FileOnlyFilter::default()
    }
}

impl DirEntryFilter for FileOnlyFilter {
    fn ignore(&self, entry: &std::fs::DirEntry) -> bool {
        match entry.file_type() {
            Ok(file_type) => file_type.is_file(),
            Err(_) => false,
        }
    }
}
