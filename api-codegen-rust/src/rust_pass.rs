use api_analyzer::Pass;
use api_analyzer::Result;
use api_parser::expressions::*;
use heck::CamelCase;

#[derive(Debug)]
pub struct RustPass;

impl RustPass {
    pub fn new() -> RustPass {
        RustPass {}
    }

    fn visit_generic_record(
        &self,
        record: &mut GenericRecordExpression,
    ) -> Option<Vec<RecordExpression>> {
        self.from_properties(&mut record.properties, &record.name)
    }

    fn visit_record(&self, record: &mut RecordExpression) -> Option<Vec<RecordExpression>> {
        self.from_properties(&mut record.properties, &record.name)
    }

    fn resolve_inner(
        &self,
        t: &mut Type,
        pname: &str,
        prop: &str,
        out: &mut Vec<RecordExpression>,
    ) -> Type {
        let name = format!("{}_{}", pname, prop).to_camel_case();

        match t {
            Type::Anonymous(a) => {
                let mut record = RecordExpression {
                    name: name.clone(),
                    location: a.location.clone(),
                    annotations: vec![],
                    properties: a.properties.clone(),
                };

                if let Some(mut more) = self.from_properties(&mut record.properties, &name) {
                    out.append(&mut more);
                }

                out.push(record);

                Type::User(name)
            }
            Type::Generic(a) => {
                let mut clone = a.clone();
                let name = clone.name.clone();
                clone.types = clone
                    .types
                    .into_iter()
                    .map(|ref mut m| {
                        self.resolve_type(m, pname, &format!("{}_{}", prop, &name), out)
                    }).collect();

                Type::Generic(clone)
            }
            _ => t.clone(),
        }
    }

    fn resolve_type(
        &self,
        t: &mut TypeExpression,
        pname: &str,
        prop: &str,
        out: &mut Vec<RecordExpression>,
    ) -> TypeExpression {
        match t {
            TypeExpression::Optional(inner) => {
                TypeExpression::Optional(self.resolve_inner(inner, pname, prop, out))
            }
            TypeExpression::Repeated(inner) => {
                TypeExpression::Repeated(self.resolve_inner(inner, pname, prop, out))
            }
            TypeExpression::Required(inner) => {
                TypeExpression::Required(self.resolve_inner(inner, pname, prop, out))
            }
        }
    }

    fn from_properties(
        &self,
        properties: &mut Vec<RecordPropertyExpression>,
        name: &str,
    ) -> Option<Vec<RecordExpression>> {
        let mut out: Vec<RecordExpression> = vec![];
        for prop in properties {
            prop.value = self.resolve_type(&mut prop.value, name, &prop.name, &mut out);
        }

        if out.is_empty() {
            return None;
        }

        Some(out)
    }

    fn visit_endpoint(&self, record: &mut HttpEndpointExpression) -> Option<Vec<RecordExpression>> {
        let mut out: Vec<RecordExpression> = vec![];

        for p in &mut record.properties {
            match p {
                HttpEndpointPropertyExpression::Returns(returns) => {
                    for r in returns {
                        r.value = self.resolve_type(&mut r.value, "", &r.name, &mut out);
                    }
                }
                HttpEndpointPropertyExpression::Body(body) => {
                    *body = self.resolve_type(body, "", "Body", &mut out);
                }
                _ => {}
            };
        }

        if out.is_empty() {
            return None;
        }

        Some(out)
    }
}

impl Pass for RustPass {
    fn execute(
        &self,
        ast: &ModuleExpression,
        _passes: &Vec<Box<dyn Pass>>,
    ) -> Result<ModuleExpression> {
        let mut clone: ModuleExpression = ast.clone();

        let mut records: Vec<_> = clone
            .body
            .iter_mut()
            .filter_map(|ref mut e| match e {
                Expression::GenericRecord(r) => self.visit_generic_record(r),
                Expression::Record(r) => self.visit_record(r),
                Expression::HttpEndpoint(r) => self.visit_endpoint(r),
                _ => None,
            }).flatten()
            .map(|m| Expression::Record(m))
            .collect();

        clone.body.append(&mut records);

        Ok(clone)
    }
}

pub fn rust() -> Box<RustPass> {
    Box::new(RustPass::new())
}
