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

#[derive(Debug, Clone)]
pub enum SourceFileType {
    RustSource(ModType),
    RustSnippet(ModStack),
    Bytes,
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
