use api_parser::expressions::{Expression, ModuleExpression};
use error::Result;
use passes::import;

pub trait Pass: Sync {
    fn execute(&self, ast: &ModuleExpression, passes: &PassList) -> Result<ModuleExpression>;
}

pub type PassList = Vec<Box<dyn Pass>>;
