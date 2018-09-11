use api_analyzer::Pass;
use api_analyzer::Result;
use api_parser::expressions::ModuleExpression;

pub struct RustPass;

impl RustPass {
    pub fn new() -> RustPass {
        RustPass {}
    }
}

impl Pass for RustPass {
    fn execute(
        &self,
        ast: &ModuleExpression,
        passes: &Vec<Box<dyn Pass>>,
    ) -> Result<ModuleExpression> {
        let mut clone: ModuleExpression = ast.clone();

        Ok(clone)
    }
}

pub fn rust() -> Box<RustPass> {
    Box::new(RustPass::new())
}
