use api_parser::expressions::GenericRecordExpression;
use visitor::vi::TypeExpressionVisitor;

pub struct GenericRecordVisitor {
    type_visitor: TypeExpressionVisitor,
}

impl GenericRecordVisitor {
    pub fn new() -> GenericRecordVisitor {
        GenericRecordVisitor {
            type_visitor: TypeExpressionVisitor::new(),
        }
    }

    pub fn visit(&self, exp: &GenericRecordExpression) -> String {
        let mut out: Vec<String> = vec![];

        let types: Vec<String> = (&exp.type_names)
            .into_iter()
            .map(|k| format!("{}: Serialize + Deserialize", k))
            .collect();

        out.push("#[derive(Debug, Default, Serialize, Deserialize)]".to_string());
        out.push(format!("pub struct {}<{}> {{", exp.name, types.join(", ")));

        let mut inner: Vec<String> = vec![];
        for prop in &exp.properties {
            let t = self.type_visitor.visit_type_expression(&prop.value);
            inner.push(format!("  {}: {}", prop.name, t));
        }
        out.push(inner.join(",\n"));
        out.push("}".to_string());

        out.join("\n")
    }
}
