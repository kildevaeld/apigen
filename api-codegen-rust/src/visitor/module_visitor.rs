use api_parser::expressions::{Expression, ModuleExpression};
use heck::CamelCase;
use std::path::PathBuf;
use template::{render_module, ModuleModel};
use visitor::vi::{EndpointVisitor, GenericRecordVisitor, RecordVisitor};

fn indent(s: &str, indent: &str) -> String {
    let mut out = vec![];
    for line in s.lines() {
        out.push(format!("{}{}", indent, line));
    }
    out.join("\n")
}

pub struct ModuleVisitor {
    record_visitor: RecordVisitor,
    generic_record_visitor: GenericRecordVisitor,
    endpoint_visitor: EndpointVisitor,
}

impl ModuleVisitor {
    pub fn new() -> ModuleVisitor {
        ModuleVisitor {
            generic_record_visitor: GenericRecordVisitor::new(),
            record_visitor: RecordVisitor::new(),
            endpoint_visitor: EndpointVisitor::new(),
        }
    }

    pub fn visit(&self, ast: &ModuleExpression) -> String {
        let mut methods: Vec<String> = vec![];
        let mut user_types: Vec<String> = vec![];

        for exp in &ast.body {
            if let Some(entry) = match exp {
                Expression::Record(record) => Some(self.record_visitor.visit(ast, &record)),
                Expression::GenericRecord(record) => {
                    Some(self.generic_record_visitor.visit(ast, &record))
                }
                Expression::HttpEndpoint(endpoint) => {
                    methods.push(indent(&self.endpoint_visitor.visit(ast, &endpoint), "  "));
                    None
                }
                _ => None,
            } {
                user_types.push(entry);
            }
        }

        let mut imports = ast.imports.iter().map(|m| m.name()).collect::<Vec<_>>();

        let path = PathBuf::from(&ast.path);
        let ext = path.extension().unwrap_or_default().to_str().unwrap();
        let name = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(&ext, "");

        let content = render_module(&ModuleModel {
            module_name: format!("{}_service", name).to_camel_case(),
            methods,
            user_types,
            imports,
        });

        content
    }
}
