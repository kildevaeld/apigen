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
            let prop_name = match prop.name.as_str() {
                "type" | "match" => {
                    format!("#[serde(rename = \"{}\")]\n  {}_", prop.name, prop.name)
                }
                s => s.to_owned(),
            };
            inner.push(format!("  {}: {}", prop_name, t));
        }
        out.push(inner.join(",\n"));
        out.push("}".to_string());

        out.join("\n")
    }
}
