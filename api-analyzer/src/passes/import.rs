use analyze::analyze_file;
use api_parser::expressions::*;
use error::{AnalyzerError, Result};
use std::path::Path;
use types::Pass;

pub struct Import;

impl Import {
    pub fn new() -> Import {
        Import {}
    }

    fn import(
        &self,
        expr: &ImportExpression,
        parent: &ModuleExpression,
        passes: &Vec<Box<dyn Pass>>,
    ) -> Result<ModuleExpression> {
        let ppath = Path::new(parent.path.as_str());
        let pdir = ppath.parent();

        if let None = pdir {
            return Err(AnalyzerError::Import("parent has no path".to_string()));
        }

        let fpath = pdir.unwrap().join(expr.path.as_str());
        let fpath_str = fpath.to_str().unwrap();

        let file = analyze_file(fpath_str, passes)?;

        Ok(file)
    }
}

impl Pass for Import {
    fn execute(
        &self,
        ast: &ModuleExpression,
        passes: &Vec<Box<dyn Pass>>,
    ) -> Result<ModuleExpression> {
        if !ast.imports.is_empty() {
            return Err(AnalyzerError::Import("Import already run".to_string()));
        } else if ast.path.is_empty() {
            return Err(AnalyzerError::Import("Path cannot be empty".to_string()));
        }

        let path = Path::new(&ast.path);

        let mut clone = ast.clone();

        for exp in &ast.body {
            match exp {
                Expression::Import(import) => {
                    let result = self.import(&import, &ast, passes)?;
                    if let Some(p) = clone.imports.iter().find(|f| f.path == result.path) {
                        println!("{:?}", p);
                        break;
                    }
                    clone.imports.push(result);
                }
                _ => {}
            };
        }

        Ok(clone)
    }
}
