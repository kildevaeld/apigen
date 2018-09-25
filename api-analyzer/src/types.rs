use api_parser::expressions::{Expression, ModuleExpression};
use error::Result;
use passes::import;
use std::fmt::Debug;

pub trait Pass: Sync + Debug {
    fn execute(&self, ast: &ModuleExpression, passes: &PassList) -> Result<ModuleExpression>;
}

pub type PassList = Vec<Box<dyn Pass>>;
