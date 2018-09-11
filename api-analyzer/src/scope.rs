use api_parser::expressions::{
    EnumExpression, Expression, GenericRecordExpression, ModuleExpression, RecordExpression,
};

#[derive(Debug)]
pub enum ScopeType<'a> {
    Record(&'a RecordExpression),
    GenericRecord(&'a GenericRecordExpression),
    Enum(&'a EnumExpression),
}

pub struct Scope<'a> {
    pub ast: &'a ModuleExpression,
    pub scope: Vec<ScopeType<'a>>,
}

fn build_scope_to<'a>(ast: &'a ModuleExpression, to: &mut Vec<ScopeType<'a>>) {
    for exp in &ast.body {
        let t = match exp {
            Expression::Record(record) => ScopeType::Record(record),
            Expression::Enum(e) => ScopeType::Enum(e),
            Expression::GenericRecord(r) => ScopeType::GenericRecord(r),
            _ => continue,
        };
        to.push(t);
    }

    for impo in &ast.imports {
        build_scope_to(impo, to);
    }
}

fn build_scope<'a>(ast: &'a ModuleExpression) -> Vec<ScopeType<'a>> {
    let mut vec = vec![];
    build_scope_to(ast, &mut vec);
    vec
}

impl<'a> Scope<'a> {
    pub fn build(ast: &'a ModuleExpression) -> Scope<'a> {
        Scope {
            ast: ast,
            scope: build_scope(ast),
        }
    }

    pub fn is_present(&self, name: &str) -> bool {
        for e in &self.scope {
            if match e {
                ScopeType::Enum(en) => en.name == name,
                ScopeType::Record(r) => r.name == name,
                ScopeType::GenericRecord(r) => r.name == name,
            } {
                return true;
            }
        }

        false
    }
}
