use api_parser::expressions::ModuleExpression;
use error::{ErrorKind, Result};
use rayon::prelude::*;
use repository::Artifact;
use std::env;
use std::fs::{canonicalize, create_dir_all, read_dir, DirEntry, File};
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct PackagePair(pub ModuleExpression, pub Vec<Artifact>);
pub type Package = Vec<PackagePair>;
use api_analyzer::{analyze_file, Pass};

pub trait CodeGenerator {
    fn transform(&self, ast: &ModuleExpression) -> Result<Vec<Artifact>>;
    fn augment_package(
        &self,
        _path: &Path,
        _modules: &Vec<ModuleExpression>,
        _artifacts: Vec<Artifact>,
    ) -> Result<Vec<Artifact>> {
        Ok(_artifacts)
    }
}

fn visit_dirs<T: AsRef<Path>>(path: T, ext: &str) -> Result<Vec<DirEntry>> {
    let mut out: Vec<DirEntry> = vec![];

    let path = path.as_ref();

    if path.is_dir() {
        for entry in read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                out.append(&mut visit_dirs(&path, ext)?);
            } else {
                if let Some(e) = path.extension() {
                    if e.to_str().unwrap() == ext {
                        out.push(entry);
                    }
                }
            }
        }
    }

    Ok(out)
}

pub fn transform_package<T, G>(
    path: T,
    generator: &G,
    passes: &Vec<Box<dyn Pass>>,
) -> Result<Vec<Artifact>>
where
    T: AsRef<Path>,
    G: CodeGenerator + Sync,
{
    let path = path.as_ref();
    let resolved_path: PathBuf;

    if !path.is_absolute() {
        resolved_path = canonicalize(path)?;
    } else {
        resolved_path = path.to_path_buf();
    }

    let files = visit_dirs(&resolved_path, "api")?;

    // Analyze and transform in parallel
    let results: Vec<_> = files
        .par_iter()
        .map(|ref file| {
            let path = file.path();
            Ok(analyze_file(&path, passes)?)
        })
        .map(|m: Result<ModuleExpression>| -> Result<PackagePair> {
            if !m.is_ok() {
                return Err(m.err().unwrap());
            }
            let m = m.unwrap();
            let artifacts = generator.transform(&m)?;
            Ok(PackagePair(m, artifacts))
        })
        .collect::<Result<Vec<PackagePair>>>()?;

    let mut modules: Vec<ModuleExpression> = (&results).into_iter().map(|m| m.0.clone()).collect();
    let mut artifacts: Vec<Artifact> = results.into_iter().flat_map(|m| m.1).collect();

    let path_string = format!("{}/", resolved_path.to_str().unwrap());

    artifacts = generator.augment_package(&resolved_path, &modules, artifacts)?;

    Ok(artifacts
        .into_iter()
        .map(|mut m| {
            m.path = PathBuf::from(m.path.to_str().unwrap().replace(&path_string, ""));
            m
        })
        .collect::<Vec<Artifact>>())
}

pub fn ensure_path<T: AsRef<Path>>(path: T) -> Result<()> {
    let path = path.as_ref();
    let exists = path.exists();
    if exists && !path.is_dir() {
        bail!(ErrorKind::Resolve(path.to_str().unwrap().to_string()));
    } else if !exists {
        create_dir_all(path)?;
    }

    Ok(())
}

pub fn write_package<T: AsRef<Path>>(package: &Vec<Artifact>, dest: T) -> Result<()> {
    let mut path = dest.as_ref().to_path_buf();
    if !path.is_absolute() {
        path = env::current_dir()?.join(path);
    }

    ensure_path(&path)?;

    for p in package {
        let file_path = path.join(&p.path);
        if let Some(parent) = file_path.parent() {
            ensure_path(parent)?;
        }

        if file_path.exists() {
            println!("file exists: {:?}", file_path);
            continue;
        }
        let mut file = File::create(file_path)?;
        file.write(&p.content)?;
        file.flush()?;
    }

    Ok(())
}
