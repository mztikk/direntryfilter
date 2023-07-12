#![deny(missing_docs)]
#![deny(rustdoc::missing_doc_code_examples)]

//! An interface for and collection of filters for DirEntry.
//!
//! This library provides a simple interface for creating and using filters.
//! Filters are used to ignore DirEntry.

/// Filter for matching only directories
mod directory_only;
/// Filter for matching only files
mod file_only;

pub use directory_only::DirectoryOnlyFilter;
pub use file_only::FileOnlyFilter;
use std::fs::DirEntry;

/// Provides an interface filtering and ignoring DirEntry.
pub trait IgnoreDirEntry {
    /// Returns `true` if the DirEntry should be ignored.
    fn ignore(&self, entry: &DirEntry) -> bool;
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// An enum that represents different types of filters for ignoring dir entries.
pub enum DirEntryFilter {
    /// Filter that matches files.
    FileOnly(FileOnlyFilter),
    /// Filter that matches directories.
    DirectoryOnly(DirectoryOnlyFilter),
}

impl From<FileOnlyFilter> for DirEntryFilter {
    fn from(value: FileOnlyFilter) -> Self {
        DirEntryFilter::FileOnly(value)
    }
}

impl From<DirectoryOnlyFilter> for DirEntryFilter {
    fn from(value: DirectoryOnlyFilter) -> Self {
        DirEntryFilter::DirectoryOnly(value)
    }
}

impl DirEntryFilter {
    /// Creates a new `DirEntryFilter` for `FileOnlyFilter`.
    ///
    /// # Examples
    ///
    /// ```
    /// use direntryfilter::DirEntryFilter;
    ///
    /// let filter = DirEntryFilter::new_file();
    /// ```
    pub fn new_file() -> Self {
        FileOnlyFilter::default().into()
    }

    /// Creates a new `DirEntryFilter` for `DirectoryOnlyFilter`.
    ///
    /// # Examples
    ///
    /// ```
    /// use direntryfilter::DirEntryFilter;
    ///
    /// let filter = DirEntryFilter::new_directory();
    /// ```
    pub fn new_directory() -> Self {
        DirectoryOnlyFilter::default().into()
    }
}

impl IgnoreDirEntry for DirEntryFilter {
    fn ignore(&self, entry: &DirEntry) -> bool {
        match self {
            DirEntryFilter::FileOnly(x) => x.ignore(entry),
            DirEntryFilter::DirectoryOnly(x) => x.ignore(entry),
        }
    }
}

impl<T: AsRef<[DirEntryFilter]>> IgnoreDirEntry for T {
    fn ignore(&self, entry: &DirEntry) -> bool {
        self.as_ref().iter().any(|filter| filter.ignore(entry))
    }
}
