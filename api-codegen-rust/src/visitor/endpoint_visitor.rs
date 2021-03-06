use api_parser::expressions::{
    HttpEndpointExpression, HttpEndpointPathExpression, HttpEndpointPropertyExpression, HttpMethod,
    ModuleExpression,
};
use heck::{CamelCase, SnakeCase};
use template::{render_method, MethodModel};
use visitor::vi::TypeExpressionVisitor;
pub struct EndpointVisitor {
    type_visitor: TypeExpressionVisitor,
}

impl EndpointVisitor {
    pub fn new() -> EndpointVisitor {
        EndpointVisitor {
            type_visitor: TypeExpressionVisitor::new(),
        }
    }

    fn method(&self, method: &HttpMethod) -> &str {
        match method {
            HttpMethod::Get => "Method::GET",
            HttpMethod::Post => "Method::POST",
            HttpMethod::Put => "Method::PUT",
            HttpMethod::Patch => "Method::PATCH",
            HttpMethod::Delete => "Method::DELETE",
            HttpMethod::Head => "Method::HEAD",
        }
    }

    pub fn visit(&self, module: &ModuleExpression, exp: &HttpEndpointExpression) -> String {
        let mut name = vec![];
        let mut paths = vec![];
        let mut arguments = vec![];
        let mut returns = String::new();
        let mut has_body = false;
        let mut has_query = false;
        let mut has_auth = false;
        let mut has_headers = false;
        let mut headers = vec![];

        for n in &exp.path {
            name.push(match n {
                HttpEndpointPathExpression::Segment(s) => {
                    paths.push(format!("\"{}\"", s));
                    s.clone()
                }
                HttpEndpointPathExpression::Param(s) => {
                    paths.push(s.clone());
                    arguments.push(format!("{}: &str", s));
                    s.clone()
                }
            });
        }

        for n in &exp.properties {
            match n {
                HttpEndpointPropertyExpression::Returns(props) => {
                    returns = self.type_visitor.visit_type_expression(module, &props);
                }
                HttpEndpointPropertyExpression::Query(query) => {
                    has_query = true;
                    arguments.push(format!(
                        "query: {}",
                        self.type_visitor.visit_type_expression(module, query)
                    ));
                }
                HttpEndpointPropertyExpression::Body(b) => {
                    has_body = true;
                    arguments.push(format!(
                        "body: {}",
                        self.type_visitor.visit_type_expression(module, b)
                    ));
                }
                HttpEndpointPropertyExpression::Auth(a) => {
                    has_auth = true;
                    arguments.push(format!("auth: auth::Authorization"));
                }
                HttpEndpointPropertyExpression::Headers(h) => {
                    has_headers = true;
                    arguments.push(format!(
                        "headers: {}",
                        format!("{}_{}", exp.name(), "Headers").to_camel_case()
                    ));
                    for p in &h.value {
                        headers.push(format!(
                            "utils::set_header(request, \"{}\", &headers.{})",
                            p.name,
                            p.name.to_snake_case()
                        ));
                    }
                }
                _ => {}
            };
        }

        //let path = name.join("/");

        render_method(&MethodModel {
            http_paths: paths.join(", "),
            http_method: self.method(&exp.method).to_string(),
            method_name: exp.name(), //name.join("_").to_snake_case(),
            method_return: returns,
            arguments: arguments.join(", "),
            has_body,
            has_query,
            has_auth,
            has_headers,
            headers,
        })
    }
}
