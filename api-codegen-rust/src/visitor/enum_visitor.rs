use api_parser::expressions::{EnumExpression, ModuleExpression};
use visitor::vi::TypeExpressionVisitor;

pub struct EnumVisitor {
    type_visitor: TypeExpressionVisitor,
}

impl EnumVisitor {
    pub fn new() -> EnumVisitor {
        EnumVisitor {
            type_visitor: TypeExpressionVisitor::new(),
        }
    }

    pub fn visit_enum(&self, module: &ModuleExpression, exp: &EnumExpression) -> String {
        let mut out: Vec<String> = vec![];
        out.push("#[derive(Debug, Default, Serialize, Deserialize)]".to_string());
        out.push(format!("pub enum {} {{", exp.name));

        let mut inner: Vec<String> = vec![];
        for prop in &exp.properties {
            //let t = self.type_visitor.visit_type_expression(module, &prop.value);
            let prop_name = match prop.name.as_str() {
                "type" | "match" => {
                    format!("#[serde(rename = \"{}\")]\n  pub {}_", prop.name, prop.name)
                }
                s => format!("{}", s),
            };
            inner.push(format!("  {} = {}", prop_name, prop.value));
        }
        out.push(inner.join(",\n"));
        out.push("}".to_string());

        out.join("\n")
    }
}
