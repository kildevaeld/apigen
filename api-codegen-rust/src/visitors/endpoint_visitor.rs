use visitors::TypeExpressionVisitor;

use api_parser::expressions::{
    HttpEndpointExpression, HttpEndpointPathExpression, HttpEndpointPropertyExpression,
    HttpEndpointReturnsExpression, HttpMethod, HttpQuery,
};
use handlebars::Handlebars;
use heck::SnakeCase;

use template::{render_method, MethodModel, METHOD};

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
        }
    }

    pub fn visit_endpoint(&self, exp: &HttpEndpointExpression) -> String {
        let mut out: Vec<String> = vec![];

        let mut name = vec![];
        let mut paths = vec![];
        let mut arguments = vec![];
        let mut returns = String::new();
        let mut successStatus = 200;
        let mut has_body = false;
        let mut has_query = false;

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
                    for ret in props {
                        let status: u16 = ret.name.parse().unwrap();

                        match status {
                            200...299 => {
                                successStatus = status;
                                returns = self.type_visitor.visit_type_expression(&ret.value)
                            }
                            _ => {}
                        };
                    }
                }
                HttpEndpointPropertyExpression::Query(query) => {
                    match query {
                        HttpQuery::Record(record) => {
                            has_query = true;
                            arguments.push(format!("query: {}", record))
                        }
                        _ => {}
                    };
                }
                HttpEndpointPropertyExpression::Body(b) => {
                    has_body = true;
                    arguments.push(format!(
                        "body: {}",
                        self.type_visitor.visit_type_expression(b)
                    ));
                }
                _ => {}
            };
        }

        let path = name.join("/");

        render_method(&MethodModel {
            http_paths: paths.join(", "),
            http_method: self.method(&exp.method).to_string(),
            method_name: name.join("_").to_snake_case(),
            method_return: returns,
            arguments: arguments.join(", "),
            has_body,
            has_query,
        })

        // let boiler = include_str!("method_boilerplate.hbs");

        // let reg = Handlebars::new();

        // reg.render_template(
        //     boiler,
        //     &json!({
        //     "httpPath": path,
        //     "httpPaths": paths.join(", "),
        //     "httpMethod": self.method(&exp.method),
        //     "methodName": name.join("_").to_snake_case(),
        //     "methodReturn": returns,
        //     "arguments": arguments.join(", "),
        //     "successStatus": successStatus,
        //     "has_body": has_body,
        //     "has_query": has_query
        // }),
        // ).unwrap()

        // let mut sig: Vec<String> = vec![];
        // sig.push(format!(
        //     "pub fn {}(client: &hyper::Client) -> Future<> {{",
        //     name.concat()
        // ));
        // out.push(sig.join(""));
        // out.push(self.boilerplate(&path, &exp));

        // // for p in &exp.properties {
        // //     match p {
        // //         HttpEndpointPropertyExpression::
        // //     }
        // // }

        // out.push("}".to_string());

        //out.join("\n")
    }
}
