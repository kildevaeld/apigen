use api_analyzer;
use api_codegen;
use api_extensions::plugins::PluginManager;
use api_extensions::{Extension, ExtensionManager};
use error::Result;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct RepositoryBuilder {
    search_paths: Vec<PathBuf>,
}

impl RepositoryBuilder {
    pub fn search_path<T: AsRef<Path>>(&mut self, path: T) -> &mut Self {
        self.search_paths.push(path.as_ref().to_path_buf());
        self
    }

    pub fn build(self) -> Repository {
        Repository {
            plugins: ExtensionManager::new(),
            search_paths: self.search_paths,
            loaded: false,
        }
    }
}

type PassesFactory = Box<Fn() -> Vec<Box<dyn api_analyzer::Pass>>>;

#[derive(Default)]
struct PluginBuilder {
    factories: HashMap<String, PassesFactory>,
}

pub struct Repository {
    plugins: ExtensionManager,
    search_paths: Vec<PathBuf>,
    loaded: bool,
}

impl Repository {
    pub fn new() -> RepositoryBuilder {
        RepositoryBuilder::default()
    }

    pub fn load(&mut self) -> Result<()> {
        if self.loaded {
            return Ok(());
        }

        for path in &self.search_paths {
            let files = fs::read_dir(&path)?;

            for file in files {
                let path = file?.path();
                self.plugins.load_plugin(&path)?;
            }
        }
        Ok(())
    }

    pub fn add_plugin(&mut self, plugin: Box<dyn Extension>) -> &Box<dyn Extension> {
        let e = self.plugins.add_plugin(plugin);
        e.instance()
    }

    pub fn list(&self) -> Vec<&Box<dyn Extension>> {
        self.plugins
            .plugins()
            .iter()
            .map(|m| m.instance())
            .collect()
    }
}
