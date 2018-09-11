use api_analyzer::{FileAst, Scope};
use bytes::Bytes;
use std::path::PathBuf;

pub struct CompilationUnit {
    files: Vec<FileAst>,
}

impl CompilationUnit {}

impl IntoIterator for CompilationUnit {
    type Item = FileAst;
    type IntoIter = ::std::vec::IntoIter<FileAst>;

    fn into_iter(self) -> Self::IntoIter {
        self.files.into_iter()
    }
}
