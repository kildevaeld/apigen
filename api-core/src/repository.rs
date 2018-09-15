use api_analyzer;
use api_codegen;
use api_extensions::plugins::PluginManager;
use api_extensions::{Extension, ExtensionBuilder, Extensions};
use error::Result;
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
            plugins: Extensions::new(),
            search_paths: self.search_paths,
            loaded: false,
        }
    }
}

struct PluginBuilder {}

impl ExtensionBuilder for PluginBuilder {
    fn register_pass(
        &mut self,
        name: &str,
        pass: Fn() -> Vec<Box<dyn api_analyzer::Pass>>,
    ) -> &mut ExtensionBuilder {
        self
    }
    fn register_codegen(
        &mut self,
        name: &str,
        pass: Box<dyn api_codegen::CodeGenerator>,
    ) -> &mut ExtensionBuilder {
        self
    }
}

pub struct Repository {
    plugins: Extensions,
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
                self.plugins.load_plugin(&path);
            }
        }
        Ok(())
    }

    pub fn list(&self) -> Vec<&Box<dyn Extension>> {
        self.plugins
            .plugins()
            .iter()
            .map(|m| m.instance())
            .collect()
    }
}
