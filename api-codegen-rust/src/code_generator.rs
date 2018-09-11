use api_codegen::{Artifact, CodeGenerator, Result};
use api_parser::expressions::{Expression, ModuleExpression};
use bytes::Bytes;
use handlebars::Handlebars;
use heck::CamelCase;
use std::path::PathBuf;
use template::{render_module, ModuleModel};
use visitors::{EndpointVisitor, GenericRecordVisitor, RecordVisitor};
pub struct RustCodeGenerator {}

impl RustCodeGenerator {
    pub fn new() -> RustCodeGenerator {
        RustCodeGenerator {}
    }
}

fn indent(s: &str, indent: &str) -> String {
    let mut out = vec![];
    for line in s.lines() {
        out.push(format!("{}{}", indent, line));
    }
    out.join("\n")
}

impl CodeGenerator for RustCodeGenerator {
    fn transform(&self, ast: &ModuleExpression) -> Result<Vec<Artifact>> {
        let mut methods: Vec<String> = vec![];
        let mut records: Vec<String> = vec![];

        let rec_v = RecordVisitor::new();
        let grec_v = GenericRecordVisitor::new();
        let end_v = EndpointVisitor::new();

        for exp in &ast.body {
            if let Some(entry) = match exp {
                Expression::Record(record) => Some(rec_v.visit_record(&record)),
                Expression::GenericRecord(record) => Some(grec_v.visit_generic_record(&record)),
                Expression::HttpEndpoint(endpoint) => {
                    methods.push(indent(&end_v.visit_endpoint(&endpoint), "  "));
                    None
                }
                _ => None,
            } {
                records.push(entry);
            }
        }

        let path = PathBuf::from(&ast.path);
        let ext = path.extension().unwrap_or_default().to_str().unwrap();
        let name = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(&ext, "");

        let content = render_module(&ModuleModel {
            methods: methods,
            module_name: name.to_camel_case(),
            user_types: records,
        });

        println!("{}", content);
        Ok(vec![Artifact {
            path: PathBuf::new(),
            content: Bytes::from(content),
        }])
    }
}
