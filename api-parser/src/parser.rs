use error::Result;
use expressions::*;
use lexer::{tokenize, Pair, Rule};
use pest;

fn parse_record(input: &Pair) -> RecordExpression {
    let span = input.clone().into_span();
    let mut record = RecordExpression {
        name: String::from(""),
        properties: vec![],
        annotations: vec![],
        location: Location(span.start(), span.end()),
    };

    let cloned = input.clone();

    for inner_pair in cloned.into_inner() {
        let span = inner_pair.clone().into_span();
        match inner_pair.as_rule() {
            Rule::type_name => record.name = String::from(span.as_str()),
            Rule::annotation => record.annotations.push(parse_annotation(&inner_pair)),
            Rule::property => {
                let prop = parse_property(&inner_pair);
                record.properties.push(prop);
            }
            _ => {}
        };
    }

    return record;
}

fn parse_import(input: &Pair) -> ImportExpression {
    let span = input.clone().into_span();

    let pair = input.clone().into_inner().next().unwrap();
    let inner_span = pair.clone().into_span();

    ImportExpression {
        path: String::from(inner_span.as_str()),
        location: Location(span.start(), span.end()),
    }
}

fn parse_annotation(input: &Pair) -> AnnotationExpression {
    let span = input.clone().into_span();

    let mut annotation = AnnotationExpression {
        name: String::from(""),
        value: JSONExpression::Bool(true),
        location: Location(span.start(), span.end()),
    };

    let cloned = input.clone();

    for pair in cloned.into_inner() {
        let span = pair.clone().into_span();
        match pair.as_rule() {
            Rule::identifier => annotation.name = String::from(span.as_str()),
            Rule::json_value => annotation.value = parse_json_value(&pair),
            _ => {}
        }
    }

    annotation
}

fn parse_json_object(input: &Pair) -> JSONExpression {
    let mut props: Vec<ObjectPropertyExpression> = vec![];

    for pair in input.clone().into_inner() {
        match pair.as_rule() {
            Rule::json_pair => {
                let span = pair.clone().into_span();
                let mut prop = ObjectPropertyExpression {
                    name: String::from(""),
                    value: JSONExpression::Null,
                    location: Location(span.start(), span.end()),
                };

                for inner in pair.clone().into_inner() {
                    let span = inner.clone().into_span();

                    match inner.as_rule() {
                        Rule::identifier => prop.name = String::from(span.as_str()),
                        Rule::json_value => prop.value = parse_json_value(&inner),
                        _ => {}
                    };
                }

                props.push(prop);
            }
            _ => {}
        };
    }
    let span = input.clone().into_span();

    JSONExpression::Object {
        location: Location(span.start(), span.end()),
        properties: props,
    }
}

fn parse_json_array(input: &Pair) {}
fn parse_json_value(input: &Pair) -> JSONExpression {
    for pair in input.clone().into_inner() {
        let exp = match pair.as_rule() {
            Rule::json_object => parse_json_object(&pair),
            _ => JSONExpression::Null,
        };
        return exp;
    }
    JSONExpression::Null
}

fn parse_enum(input: &Pair) -> EnumExpression {
    let span = input.clone().into_span();

    let mut e = EnumExpression {
        name: String::from(""),
        variants: vec![],
        location: Location(span.start(), span.end()),
    };

    for pair in input.clone().into_inner() {
        let span = pair.clone().into_span();
        match pair.as_rule() {
            Rule::type_name => e.name = String::from(span.as_str()),
            Rule::identifier => e.variants.push(String::from(span.as_str())),
            _ => {}
        };
    }

    e
}

// fn parse_type(input: &str) -> TypeExpression {
//     TypeExpression {
//         optional: false,
//         value: match input {
//             "int64" => Type::Int64,
//             "int32" => Type::Int32,
//             "int16" => Type::Int16,
//             "int8" => Type::Int8,
//             "uint64" => Type::Uint64,
//             "uint32" => Type::Uint32,
//             "uint16" => Type::Uint16,
//             "uint8" => Type::Uint8,
//             "float" => Type::Float,
//             "double" => Type::Double,
//             "string" => Type::String,
//             "bytes" => Type::Bytes,
//             "bool" => Type::Bool,
//             _ => Type::User(String::from(input)),
//         },
//     }
// }

fn parse_generic_type(input: &Pair) -> Type {
    let mut types = vec![];
    let mut name = String::from("");

    for pair in input.clone().into_inner() {
        let span = pair.clone().into_span();
        match pair.as_rule() {
            Rule::type_name => name = String::from(span.as_str()),
            Rule::anonymous_record_type | Rule::type_builtins => {
                types.push(parse_all_type_exp(&pair));
            }

            _ => {}
        };
    }

    Type::Generic(GenericExpression {
        name: name,
        types: types,
    })
}

fn parse_builtin_type(input: &str) -> Type {
    let t = match input {
        "int64" => Builtin::Int64,
        "int32" => Builtin::Int32,
        "int16" => Builtin::Int16,
        "int8" => Builtin::Int8,
        "uint64" => Builtin::Uint64,
        "uint32" => Builtin::Uint32,
        "uint16" => Builtin::Uint16,
        "uint8" => Builtin::Uint8,
        "float" => Builtin::Float,
        "double" => Builtin::Double,
        "string" => Builtin::String,
        "bytes" => Builtin::Bytes,
        "bool" => Builtin::Bool,
        "map" => Builtin::Map,
        _ => Builtin::Void,
    };
    Type::Builtin(t)
}

fn parse_anonymous_record_type(input: &Pair) -> Type {
    let span = input.clone().into_span();
    let mut exp = AnonymousRecordExpression {
        location: Location(span.start(), span.end()),
        properties: vec![],
    };

    for pair in input.clone().into_inner() {
        match pair.as_rule() {
            Rule::property => exp.properties.push(parse_property(&pair)),
            _ => {}
        };
    }

    Type::Anonymous(exp)
}

fn parse_all_type_exp(input: &Pair) -> TypeExpression {
    //println!("{:?}", input);

    match input.as_rule() {
        Rule::generic_type
        | Rule::type_builtins
        | Rule::type_name
        | Rule::anonymous_record_type => TypeExpression::Required(parse_all_type(&input)),
        /*Rule::generic_type => TypeExpression::Required(parse_generic_type(&input)),
        Rule::type_builtins => TypeExpression::Required(parse_builtin_type(span.as_str())),
        Rule::type_name => TypeExpression::Required(Type::User(String::from(span.as_str()))),
        Rule::anonymous_record_type => {
            TypeExpression::Required(parse_anonymous_record_type(&input))
        }*/
        Rule::optional_type => {
            let pair = input.clone().into_inner().next().unwrap();
            TypeExpression::Optional(parse_all_type(&pair))
        }
        Rule::repeated_type => {
            let pair = input.clone().into_inner().next().unwrap();
            TypeExpression::Repeated(parse_all_type(&pair))
        }
        _ => TypeExpression::Required(Type::Builtin(Builtin::Void)),
    }
}

fn parse_all_type(input: &Pair) -> Type {
    let span = input.clone().into_span();
    match input.as_rule() {
        Rule::generic_type => parse_generic_type(&input),
        Rule::type_builtins => parse_builtin_type(span.as_str()),
        Rule::type_name => Type::User(String::from(span.as_str())),
        Rule::anonymous_record_type => parse_anonymous_record_type(&input),
        _ => Type::Builtin(Builtin::Void),
    }
}

fn parse_property(input: &Pair) -> RecordPropertyExpression {
    let span = input.clone().into_span();
    let mut prop = RecordPropertyExpression {
        name: String::from(""),
        value: TypeExpression::Required(Type::Builtin(Builtin::Void)),
        //annotations: vec![],
        location: Location(span.start(), span.end()),
    };

    let cloned = input.clone();

    for pair in cloned.into_inner() {
        let span = pair.clone().into_span();
        match pair.as_rule() {
            Rule::identifier => prop.name = String::from(span.as_str()),
            Rule::type_builtins | Rule::type_name | Rule::anonymous_record_type => {
                prop.value = TypeExpression::Required(parse_all_type(&pair));
            }
            Rule::optional_type => {
                let pair = pair.clone().into_inner().next().unwrap();
                prop.value = TypeExpression::Optional(parse_all_type(&pair));
            }
            Rule::repeated_type => {
                let pair = pair.clone().into_inner().next().unwrap();
                prop.value = TypeExpression::Repeated(parse_all_type(&pair));
            }
            _ => {}
        };
    }

    prop
}

fn parse_http_endpoint_verb(input: &pest::Span) -> HttpMethod {
    match input.as_str() {
        "get" => HttpMethod::Get,
        "post" => HttpMethod::Post,
        "put" => HttpMethod::Put,
        "patch" => HttpMethod::Patch,
        "delete" => HttpMethod::Delete,
        _ => HttpMethod::Get,
    }
}

fn parse_http_endpoint_path(input: &Pair) -> Vec<HttpEndpointPathExpression> {
    let mut out = vec![];

    for pair in input.clone().into_inner() {
        let span = pair.clone().into_span();
        match pair.as_rule() {
            Rule::http_path_segment_part => {
                out.push(HttpEndpointPathExpression::Segment(String::from(
                    span.as_str(),
                )));
            }
            Rule::http_path_segment_param => {
                out.push(HttpEndpointPathExpression::Param(String::from(
                    span.as_str().trim_left_matches(':').to_string(),
                )));
            }
            _ => {}
        }
    }

    out
}

fn parse_http_endpoint_returns(input: &Pair) -> Vec<HttpEndpointReturnsExpression> {
    let span = input.clone().into_span();

    let mut returns = HttpEndpointReturnsExpression {
        name: String::from(""),
        location: Location(span.start(), span.end()),
        value: TypeExpression::Required(Type::Builtin(Builtin::Void)),
    };

    let mut output = vec![];

    for pair in input.clone().into_inner() {
        let span = pair.clone().into_span();
        match pair.as_rule() {
            Rule::http_status_code => returns.name = String::from(span.as_str()),
            Rule::generic_type | Rule::anonymous_record_type => {
                returns.value = parse_all_type_exp(&pair);
                output.push(returns.clone());
            }
            _ => {}
        };
    }

    output
}

fn parse_http_endpoint(input: &Pair) -> HttpEndpointExpression {
    let span = input.clone().into_span();
    let mut endpoint = HttpEndpointExpression {
        method: HttpMethod::Get,
        path: vec![],
        properties: vec![],
        location: Location(span.start(), span.end()),
    };

    let cloned = input.clone();

    for pair in cloned.into_inner() {
        let span = pair.clone().into_span();
        match pair.as_rule() {
            Rule::http_method => endpoint.method = parse_http_endpoint_verb(&span),
            Rule::http_path => endpoint.path = parse_http_endpoint_path(&pair),
            Rule::http_endpoint_returns => {
                endpoint
                    .properties
                    .push(HttpEndpointPropertyExpression::Returns(
                        parse_http_endpoint_returns(&pair),
                    ));
            }
            Rule::http_endpoint_body => {
                let inner = pair.clone().into_inner().next().unwrap();

                endpoint
                    .properties
                    .push(HttpEndpointPropertyExpression::Body(parse_all_type_exp(
                        &inner,
                    )));
            }
            Rule::http_endpoint_description => endpoint.properties.push(
                HttpEndpointPropertyExpression::Description(span.as_str().trim().to_string()),
            ),
            _ => {}
        };
    }

    endpoint
}

pub fn build_ast(input: &str) -> Result<ModuleExpression> {
    build_ast_path(input, "")
}

pub fn build_ast_path(input: &str, path: &str) -> Result<ModuleExpression> {
    let pairs = tokenize(input)?;
    let mut exps: Vec<Expression> = vec![];

    for pair in pairs {
        let exp = match pair.as_rule() {
            Rule::record_type => Expression::Record(parse_record(&pair)),
            Rule::import => Expression::Import(parse_import(&pair)),
            Rule::enum_type => Expression::Enum(parse_enum(&pair)),
            Rule::http_endpoint => Expression::HttpEndpoint(parse_http_endpoint(&pair)),
            _ => Expression::Invalid,
        };

        if exp == Expression::Invalid {
            continue;
        }
        exps.push(exp)
    }

    Ok(ModuleExpression {
        body: exps,
        imports: vec![],
        path: path.to_owned(),
    })
}
