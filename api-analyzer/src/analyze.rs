use api_parser::build_ast;
use api_parser::expressions::ModuleExpression;
use common::read_file;
use error::Result;
use passes::{import, typevalidator};
use types::Pass;

use std::fs;

#[allow(dead_code)]
pub fn default_passes() -> Vec<Box<dyn Pass>> {
    vec![
        Box::new(import::Import::new()),
        Box::new(typevalidator::TypeValidator::new()),
    ]
}

pub fn analyze_file(file: &str, passes: &Vec<Box<dyn Pass>>) -> Result<ModuleExpression> {
    let resolve_file = fs::canonicalize(file)?;

    let data = read_file(resolve_file.to_str().unwrap())?;
    let ast = build_ast(data.as_str())?;

    return analyze(resolve_file.to_str().unwrap(), ast, passes);
}

pub fn analyze(
    file_name: &str,
    ast: ModuleExpression,
    passes: &Vec<Box<dyn Pass>>,
) -> Result<ModuleExpression> {
    let mut file = ast.clone();
    file.path = file_name.to_string();

    for pass in passes {
        file = pass.execute(&file, passes)?;
    }

    Ok(file)
}
