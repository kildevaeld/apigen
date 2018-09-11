use api_parser::expressions::{Expression, ModuleExpression};
use error::Result;
use passes::import;

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct FileAst {
//     pub path: String,
//     pub imports: Vec<FileAst>,
//     pub body: Vec<Expression>,
// }

// impl IntoIterator for FileAst {
//     type Item = Expression;
//     type IntoIter = ::std::vec::IntoIter<Expression>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.body.into_iter()
//     }
// }

pub trait Pass {
    fn execute(
        &self,
        ast: &ModuleExpression,
        passes: &Vec<Box<dyn Pass>>,
    ) -> Result<ModuleExpression>;
}
