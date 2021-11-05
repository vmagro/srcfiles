use crate::mod_path::ModStack;
use std::path::PathBuf;

/// Type of module paths
#[derive(Debug, Clone, Copy)]
pub enum ModType {
    /// Module named "modname.rs"
    Adjacent,
    /// Module named "modname/mod.rs"
    ModRs,
}

/// Type of source file.
#[derive(Debug, Clone)]
pub enum SourceFileType {
    /// Rust source module.
    RustSource(ModType),
    /// Included Rust code (via `include!`).
    RustSnippet(ModStack),
    /// Included bytes (via `include_bytes!`).
    Bytes,
    /// Included string (via `include_str!`).
    String,
}

#[derive(Debug, Clone)]
pub struct SourceFileDesc {
    pub path: PathBuf,
    pub file_type: SourceFileType,
    pub parent_file: Option<PathBuf>,
}

impl SourceFileDesc {
    pub fn new(path: PathBuf, file_type: SourceFileType, parent_file: Option<PathBuf>) -> Self {
        SourceFileDesc {
            path,
            file_type,
            parent_file,
        }
    }
}
