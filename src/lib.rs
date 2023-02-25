#![deny(missing_docs)]
#![deny(rustdoc::missing_doc_code_examples)]

//! An interface for and collection of filters for DirEntry.
//!
//! This library provides a simple interface for creating and using filters.
//! Filters are used to ignore DirEntry.

use std::fs::DirEntry;

/// Filter for matching only directories
pub mod directory_only;
/// Filter for matching only files
pub mod file_only;

/// Provides an interface filtering and ignoring DirEntry.
pub trait DirEntryFilter {
    /// Returns `true` if the path should be ignored.
    fn ignore(&self, entry: &DirEntry) -> bool;
}
