use api_analyzer::PassList;
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct GeneratorBuilder {
    pub(crate) dest: Option<PathBuf>,
    pub(crate) name: String,
    pub(crate) source: PathBuf,
    pub(crate) passes: PassList,
}

impl GeneratorBuilder {
    pub fn new<T: AsRef<Path>, S: AsRef<str>>(source: T, name: S) -> GeneratorBuilder {
        GeneratorBuilder {
            source: source.as_ref().to_path_buf(),
            name: name.as_ref().to_string(),
            dest: None,
            passes: vec![],
        }
    }

    pub fn dest<T: AsRef<Path>>(&mut self, dest: T) -> &mut Self {
        self.dest = Some(dest.as_ref().to_path_buf());
        self
    }

    pub fn name<T: AsRef<str>>(&mut self, dest: T) -> &mut Self {
        self.name = dest.as_ref().to_string();
        self
    }

    pub fn source<T: AsRef<Path>>(&mut self, dest: T) -> &mut Self {
        self.source = dest.as_ref().to_path_buf();
        self
    }

    pub fn passes(&mut self, dest: PassList) -> &mut Self {
        self.passes = dest;
        self
    }
}
