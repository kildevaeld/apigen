use api_parser::expressions::GenericRecordExpression;
use visitors::TypeExpressionVisitor;

pub struct GenericRecordVisitor {
    type_visitor: TypeExpressionVisitor,
}

impl GenericRecordVisitor {
    pub fn new() -> GenericRecordVisitor {
        GenericRecordVisitor {
            type_visitor: TypeExpressionVisitor::new(),
        }
    }

    pub fn visit_generic_record(&self, exp: &GenericRecordExpression) -> String {
        let mut out: Vec<String> = vec![];
        out.push("#[derive(Debug, Default, Serialize, Deserialize)]".to_string());
        out.push(format!("pub struct {} {{", exp.name));

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
