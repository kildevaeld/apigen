use api_parser::expressions::{ModuleExpression, RecordExpression};
use visitor::vi::TypeExpressionVisitor;

pub struct RecordVisitor {
    type_visitor: TypeExpressionVisitor,
}

impl RecordVisitor {
    pub fn new() -> RecordVisitor {
        RecordVisitor {
            type_visitor: TypeExpressionVisitor::new(),
        }
    }

    pub fn visit(&self, module: &ModuleExpression, exp: &RecordExpression) -> String {
        let mut out: Vec<String> = vec![];
        out.push("#[derive(Debug, Default, Serialize, Deserialize)]".to_string());
        out.push(format!("pub struct {} {{", exp.name));

        let mut inner: Vec<String> = vec![];
        for prop in &exp.properties {
            let t = self.type_visitor.visit_type_expression(module, &prop.value);
            inner.push(format!("  {}: {}", prop.name, t));
        }
        out.push(inner.join(",\n"));
        out.push("}".to_string());

        out.join("\n")
    }
}
