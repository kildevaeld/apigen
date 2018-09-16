use api_analyzer::{analyze_file, default_passes, Pass};
use api_codegen::{transform_package_boxed, write_package, Artifact, CodeGenerator, Package};
use api_parser::expressions::ModuleExpression;
use error::{ErrorKind, Result};
use generator::GeneratorBuilder;
use repository::{Repository, RepositoryBuilder};
use std::path::Path;
#[derive(Default)]
pub struct CoreBuilder {
    repo: RepositoryBuilder,
}

impl CoreBuilder {
    pub fn search_path<T: AsRef<Path>>(mut self, path: T) -> CoreBuilder {
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

    pub fn repository_mut(&mut self) -> &mut Repository {
        &mut self.repo
    }

    pub fn ast<T: AsRef<str>, P: AsRef<Path>>(
        &self,
        name: T,
        file_name: P,
    ) -> Result<ModuleExpression> {
        self.ast_with_passes(name, file_name, default_passes())
    }

    pub fn ast_with_passes<T: AsRef<str>, P: AsRef<Path>>(
        &self,
        name: T,
        file_name: P,
        mut passes: Vec<Box<dyn Pass>>,
    ) -> Result<ModuleExpression> {
        let name = name.as_ref();
        if let Some(found) = self.repo.list().into_iter().find(|m| m.name() == name) {
            if let Some(mut fpass) = found.passes() {
                passes.append(&mut fpass);
            }

            return Ok(analyze_file(file_name, &passes)?);
        }

        Err(ErrorKind::Pass(format!("could not find {}", name)).into())
    }

    pub fn gen_code(&self, mut builder: GeneratorBuilder) -> Result<ModuleExpression> {
        // let name = name.as_ref();

        let found = match self
            .repo
            .list()
            .into_iter()
            .find(|m| m.name() == builder.name)
        {
            Some(found) => found,
            None => return Err(ErrorKind::Pass(format!("could not find {}", builder.name)).into()),
        };

        if let Some(mut fpass) = found.passes() {
            builder.passes.append(&mut fpass);
        }

        let generator = match found.generator() {
            Some(found) => found,
            None => {
                return Err(ErrorKind::Pass(format!("no generator for {}", builder.name)).into())
            }
        };

        let artifacts: Vec<Artifact>;
        if builder.source.is_dir() {
            artifacts = transform_package_boxed(&builder.source, &generator, &builder.passes)?
        } else {
            let ast = analyze_file(&builder.source, &builder.passes)?;
            artifacts = generator.transform(&ast)?;
        }

        if let Some(dest) = builder.dest {
            write_package(&artifacts, &dest)?;
        }

        Err(ErrorKind::Pass(format!("could not find {}", builder.name)).into())
    }
}
