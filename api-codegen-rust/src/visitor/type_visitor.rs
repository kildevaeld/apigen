use api_parser::expressions::{
    AnonymousRecordExpression, Builtin, GenericExpression, ModuleExpression, Type, TypeExpression,
    UserType,
};

pub struct TypeExpressionVisitor;

impl TypeExpressionVisitor {
    pub fn new() -> TypeExpressionVisitor {
        TypeExpressionVisitor {}
    }

    pub fn visit_builtin(&self, exp: &Builtin) -> String {
        let m = match exp {
            Builtin::Int8 => "i8",
            Builtin::Uint8 => "u8",
            Builtin::Int16 => "i16",
            Builtin::Uint16 => "u16",
            Builtin::Int32 => "i32",
            Builtin::Uint32 => "u32",
            Builtin::Int64 => "i64",
            Builtin::Uint64 => "u64",
            Builtin::Float => "f64",
            Builtin::Bool => "bool",
            Builtin::String => "String",
            Builtin::Object => "api_support::Object",
            Builtin::Void => "()",
            Builtin::Bytes => "Vec<u8>",
            Builtin::Any => "api_support::Any",
            _ => "",
        };
        m.to_string()
    }

    pub fn visit_generic(&self, module: &ModuleExpression, exp: &GenericExpression) -> String {
        let mut out: Vec<String> = vec![];

        for t in &exp.types {
            out.push(self.visit_type_expression(module, &t));
        }

        let o = format!("{}<{}>", exp.name, out.join(", "));
        let imported = module.imported_scope();

        if let Some(m) = imported.iter().find(|m| m.name() == exp.name) {
            let imported = module
                .imports
                .iter()
                .find(|i| i.local_scope().iter().find(|mm| mm == &m).is_some());
            return format!("{}::{}", imported.unwrap().name(), o);
        }
        o
    }

    pub fn visit_anonymous_type(&self, _exp: &AnonymousRecordExpression) -> String {
        panic!("ananonymous_type not handled by pass");
    }

    pub fn visit_type(&self, module: &ModuleExpression, exp: &Type) -> String {
        match exp {
            Type::Builtin(b) => self.visit_builtin(b),
            Type::Generic(b) => self.visit_generic(module, b),
            Type::User(b) => {
                let imported = module.imported_scope();

                if let Some(m) = imported.iter().find(|m| m.name() == *b) {
                    let imported = module
                        .imports
                        .iter()
                        .find(|i| i.local_scope().iter().find(|mm| mm == &m).is_some());
                    return format!("{}::{}", imported.unwrap().name(), m.name());
                }
                b.clone()
            }
            Type::Array(e) => format!("Vec<{}>", self.visit_type_expression(module, &e.value)),
            Type::Anonymous(b) => self.visit_anonymous_type(b),
        }
    }

    pub fn visit_type_expression(&self, module: &ModuleExpression, exp: &TypeExpression) -> String {
        match exp {
            TypeExpression::Optional(inner) => {
                format!("Option<{}>", self.visit_type(module, inner))
            }
            //TypeExpression::Repeated(inner) => format!("Vec<{}>", self.visit_type(module, inner)),
            TypeExpression::Required(inner) => self.visit_type(module, inner),
        }
    }

    // fn visit_type_expression_with_parent(&self, exp: &TypeExpression) -> String {
    //     match exp {
    //         TypeExpression::Optional(inner) => format!("Option<{}>", self.visit_type(inner)),
    //         TypeExpression::Repeated(inner) => format!("Vec<{}>", self.visit_type(inner)),
    //         TypeExpression::Required(inner) => self.visit_type(inner),
    //     }
    // }
}
