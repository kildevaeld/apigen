use api_parser::expressions::ModuleExpression;
use bytes::Bytes;

use code_generator::CodeGenerator;
use error::Result;
use plugin_manager::{Plugin, PluginManager};
use std::path::{Path, PathBuf};
use std::str;
use uuid::Uuid;
pub struct CodeGeneratorPluginBuilder {}

pub trait CodeGeneratorPlugin {
    fn register(&self, builder: &mut CodeGeneratorPluginBuilder) -> Result<()>;
}

pub struct Artifact {
    pub path: PathBuf,
    pub content: Bytes,
}

impl Artifact {
    pub fn new() -> Artifact {
        Artifact {
            path: PathBuf::new(),
            content: Bytes::new(),
        }
    }

    pub fn set_path<T: AsRef<Path>>(&mut self, s: T) {
        self.path = s.as_ref().to_path_buf();
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn path_mut(&mut self) -> &mut PathBuf {
        &mut self.path
    }

    pub fn as_str(&self) -> &str {
        str::from_utf8(self.content.as_ref()).unwrap()
    }
}

pub struct RepositoryEntry {
    id: Uuid,
    generator: Box<dyn CodeGenerator>,
    description: Option<String>,
    name: String,
}

pub struct Repository {
    plugins: PluginManager,
    generators: Vec<RepositoryEntry>,
}

impl Repository {
    pub fn new() -> Repository {
        Repository {
            generators: Vec::new(),
            plugins: PluginManager::new(),
        }
    }

    pub fn add(&mut self, name: &str, generator: Box<dyn CodeGenerator>) {
        self.generators.push(RepositoryEntry {
            id: Uuid::new_v4(),
            name: name.to_string(),
            generator: generator,
            description: None,
        });
    }

    pub fn add_from_path(&mut self, filename: &str) {
        unsafe {
            self.plugins.load_plugin(filename);
        }
    }

    // pub fn add_search_path(path: &str) -> Result<()> {
    //     Ok(())
    // }

    // pub fn load_generators() -> Result<()> {
    //     Ok(())
    // }
}

impl IntoIterator for Repository {
    type Item = RepositoryEntry;
    type IntoIter = ::std::vec::IntoIter<RepositoryEntry>;

    fn into_iter(self) -> Self::IntoIter {
        self.generators.into_iter()
    }
}
