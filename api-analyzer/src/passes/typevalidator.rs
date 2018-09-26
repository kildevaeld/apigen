use api_parser::expressions::{
    AnonymousRecordExpression, EnumExpression, Expression, GenericExpression,
    GenericRecordExpression, HttpEndpointExpression, HttpEndpointPropertyExpression,
    ModuleExpression, RecordExpression, Type as ApiType, TypeExpression,
};
use error::{AnalyzerError, Result};
use types::Pass;

#[derive(Debug)]
enum Type<'a> {
    Record(&'a RecordExpression),
    GenericRecord(&'a GenericRecordExpression),
    Enum(&'a EnumExpression),
}

#[derive(Debug)]

pub struct TypeValidator;

impl TypeValidator {
    pub fn new() -> TypeValidator {
        TypeValidator {}
    }

    fn build_scope_to<'a>(&self, ast: &'a ModuleExpression, to: &mut Vec<Type<'a>>) {
        for exp in &ast.body {
            let t = match exp {
                Expression::Record(record) => Type::Record(record),
                Expression::Enum(e) => Type::Enum(e),
                Expression::GenericRecord(r) => Type::GenericRecord(r),
                _ => continue,
            };
            to.push(t);
        }

        for impo in &ast.imports {
            self.build_scope_to(impo, to);
        }
    }

    fn build_scope<'a>(&self, ast: &'a ModuleExpression) -> Vec<Type<'a>> {
        let mut vec = vec![];

        self.build_scope_to(ast, &mut vec);

        vec
    }

    fn visit_generic_record(
        &self,
        expr: &GenericRecordExpression,
        scope: &Vec<Type>,
    ) -> Result<()> {
        Ok(())
    }

    fn visit_user_type(&self, name: &String, scope: &Vec<Type>) -> Result<()> {
        let found = scope.into_iter().find(|v| match v {
            Type::Enum(e) => &e.name == name,
            Type::GenericRecord(e) => &e.name == name,
            Type::Record(e) => &e.name == name,
            _ => false,
        });

        if let None = found {
            return Err(AnalyzerError::Reference(format!(
                "'{}' not found in scope",
                name
            )));
        }

        Ok(())
    }

    fn visit_generic_type(&self, expr: &GenericExpression, scope: &Vec<Type>) -> Result<()> {
        let found = scope.into_iter().find(|v| match v {
            Type::GenericRecord(e) => e.name == expr.name,
            _ => false,
        });

        if let None = found {
            return Err(AnalyzerError::Reference(format!(
                "'{}' not found in scope",
                expr.name
            )));
        }

        let found = found.unwrap();

        if let Type::GenericRecord(found) = found {
            if found.properties.len() != expr.types.len() {
                return Err(AnalyzerError::TypeError(format!(
                    "'{}' invalid number of argumnents",
                    expr.name
                )));
            }
        }

        Ok(())
    }

    fn visit_anonymous(&self, expr: &AnonymousRecordExpression, scope: &Vec<Type>) -> Result<()> {
        for prop in &expr.properties {
            let t = match &prop.value {
                TypeExpression::Optional(o) => o,
                TypeExpression::Required(o) => o,
                TypeExpression::Repeated(o) => o,
            };
            self.visit_type(&t, &scope)?;
        }
        Ok(())
    }

    fn visit_type(&self, expr: &ApiType, scope: &Vec<Type>) -> Result<()> {
        match expr {
            ApiType::User(name) => self.visit_user_type(&name, &scope)?,
            ApiType::Generic(generic) => self.visit_generic_type(&generic, &scope)?,
            ApiType::Anonymous(record) => self.visit_anonymous(&record, &scope)?,
            _ => {}
        };
        Ok(())
    }

    fn visit_record(&self, expr: &RecordExpression, scope: &Vec<Type>) -> Result<()> {
        for prop in &expr.properties {
            let t = match &prop.value {
                TypeExpression::Optional(o) => o,
                TypeExpression::Required(o) => o,
                TypeExpression::Repeated(o) => o,
            };
            self.visit_type(&t, &scope)?;
        }

        Ok(())
    }

    // fn visit_endpoint_returns(
    //     &self,
    //     expr: &HttpEndpointReturnsExpression,
    //     scope: &Vec<Type>,
    // ) -> Result<()> {
    //     Ok(())
    // }

    fn visit_endpoint<'a>(
        &self,
        expr: &HttpEndpointExpression,
        scope: &Vec<Type<'a>>,
    ) -> Result<()> {
        for p in &expr.properties {
            match p {
                HttpEndpointPropertyExpression::Returns(returns) => {
                    let t = match &returns {
                        TypeExpression::Optional(o) => o,
                        TypeExpression::Required(o) => o,
                        TypeExpression::Repeated(o) => o,
                    };
                    self.visit_type(&t, &scope)?;
                    // for r in returns {
                    //     self.visit_endpoint_returns(&r, &scope)?
                    // }
                }
                _ => {}
            };
        }

        Ok(())
    }
}

impl Pass for TypeValidator {
    fn execute(
        &self,
        ast: &ModuleExpression,
        _passes: &Vec<Box<dyn Pass>>,
    ) -> Result<ModuleExpression> {
        let clone = ast.clone();

        let scope = self.build_scope(ast);

        for exp in &ast.body {
            match exp {
                Expression::GenericRecord(record) => self.visit_generic_record(&record, &scope)?,
                Expression::Record(record) => self.visit_record(&record, &scope)?,
                Expression::HttpEndpoint(endpoint) => self.visit_endpoint(&endpoint, &scope)?,
                _ => {}
            };
        }

        Ok(clone)
    }
}
