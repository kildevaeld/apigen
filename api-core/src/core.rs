use api_extensions::plugins::PluginManager;
use api_parser::expressions::ModuleExpression;
use error::{ErrorKind, Result};
use repository::{Repository, RepositoryBuilder};
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct CoreBuilder {
    repo: RepositoryBuilder,
}

impl CoreBuilder {
    pub fn search_path<T: AsRef<Path>>(&mut self, path: T) -> &mut Self {
        self.repo.search_path(path);
        self
    }

    pub fn build(self) -> Core {
        Core {
            repo: self.repo.build(),
        }
    }
}

pub struct Core {
    repo: Repository,
}

impl Core {
    pub fn new() -> CoreBuilder {
        CoreBuilder::default()
    }

    pub fn repository(&self) -> &Repository {
        &self.repo
    }

    pub fn ast<T: AsRef<str>, P: AsRef<Path>>(
        &self,
        name: T,
        file_name: P,
    ) -> Result<ModuleExpression> {
        let name = name.as_ref();
        if let Some(found) = self
            .repo
            .list()
            .into_iter()
            .find(|m| m.name().unwrap() == name)
        {
            let passes = found.passes();
        }

        Err(ErrorKind::Pass(String::from("coudl not find")).into())
    }
}
