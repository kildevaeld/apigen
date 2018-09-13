use api_codegen::{Artifact, CodeGenerator, Result};
use api_parser::expressions::ModuleExpression;
use bytes::Bytes;
use heck::SnakeCase;
use std::path::{Path, PathBuf};
use template::{render_cargo, render_lib, CargoModel, LibModel};
use visitor::ModuleVisitor;
pub struct RustCodeGenerator {
    visitor: ModuleVisitor,
}

impl RustCodeGenerator {
    pub fn new() -> RustCodeGenerator {
        RustCodeGenerator {
            visitor: ModuleVisitor::new(),
        }
    }

    fn fix_paths(&self, artifacts: &mut Vec<Artifact>) {
        for a in artifacts {
            let path: PathBuf = a.path.clone();
            let file_name = path.file_name().unwrap();
            let mut dir_name = path.parent().unwrap().to_path_buf();
            dir_name.push("src");
            dir_name.push(file_name);
            a.path = dir_name;
        }
    }
}

// fn indent(s: &str, indent: &str) -> String {
//     let mut out = vec![];
//     for line in s.lines() {
//         out.push(format!("{}{}", indent, line));
//     }
//     out.join("\n")
// }

impl CodeGenerator for RustCodeGenerator {
    fn transform(&self, ast: &ModuleExpression) -> Result<Vec<Artifact>> {
        let content = self.visitor.visit(ast);

        let mut path = PathBuf::from(&ast.path);
        path.set_extension("rs");

        Ok(vec![Artifact {
            path: path,
            content: Bytes::from(content),
        }])
    }

    fn augment_package(
        &self,
        path: &Path,
        modules: &Vec<ModuleExpression>,
        artifacts: Vec<Artifact>,
    ) -> Result<Vec<Artifact>> {
        let mut mods = vec![];

        let mut artifacts = artifacts;
        for m in modules {
            let ext = m.path.extension().unwrap().to_str().unwrap();

            let name = m
                .path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .replace(&ext, "");

            mods.push(name.to_snake_case());
        }

        self.fix_paths(&mut artifacts);

        artifacts.push(Artifact {
            path: path.join("src/lib.rs"),
            content: Bytes::from(render_lib(&LibModel { modules: mods })),
        });

        artifacts.push(Artifact {
            path: path.join("cargo.toml"),
            content: Bytes::from(render_cargo(&CargoModel {
                name: "package".to_owned(),
            })),
        });

        Ok(artifacts)
    }
}
