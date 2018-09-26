use api_parser::expressions::ModuleExpression;
use error::Result;
use std::fmt::Debug;

pub trait Pass: Sync + Debug {
    fn execute(&self, ast: &ModuleExpression, passes: &PassList) -> Result<ModuleExpression>;
}

pub type PassList = Vec<Box<dyn Pass>>;
