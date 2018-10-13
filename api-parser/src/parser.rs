use error::Result;
use expressions::*;
use lexer::{tokenize, Pair, Rule};
use pest;
use std::path::Path;

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

fn parse_generic_record_type(input: &Pair) -> GenericRecordExpression {
    let span = input.clone().into_span();
    let mut record = GenericRecordExpression {
        name: String::from(""),
        properties: vec![],
        type_names: vec![],
        location: Location(span.start(), span.end()),
    };

    let cloned = input.clone();

    for pair in cloned.into_inner() {
        let span = pair.clone().into_span();
        match pair.as_rule() {
            Rule::type_name => record.name = String::from(span.as_str()),
            Rule::generic_type_parameters => {
                for inner_pair in pair.clone().into_inner() {
                    let inner_span = inner_pair.clone().into_span();
                    match inner_pair.as_rule() {
                        Rule::identifier => record.type_names.push(inner_span.as_str().to_string()),
                        _ => {}
                    };
                }
            }
            //Rule::annotation => record.annotations.push(parse_annotation(&pair)),
            Rule::property => {
                let prop = parse_property(&pair);
                record.properties.push(prop);
            }
            _ => {}
        };
    }

    record
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

//fn parse_json_array(input: &Pair) {}

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

fn parse_enum_property(input: &Pair, parent: i32) -> EnumPropertyExpression {
    let span = input.clone().into_span();

    let value = parent + 1;

    let mut e = EnumPropertyExpression {
        name: String::from(""),
        value: value,
        location: Location(span.start(), span.end()),
    };

    match input.as_rule() {
        //Rule::indetifier => e.name = String::from(span.as_str()),
        Rule::enum_type_body_auto => e.name = span.as_str().to_owned(),
        Rule::enum_type_body_assign => {
            let inner = input.clone().into_inner();
            for pair in inner {
                let span = pair.clone().into_span();
                match pair.as_rule() {
                    Rule::identifier => e.name = span.as_str().to_owned(),
                    Rule::number => e.value = span.as_str().parse().unwrap(),
                    _ => {}
                };
            }
            // e.properties.push(parse_enum_property(&pair, last_value));
            // last_value = e.properties.last().unwrap().value;
        }
        //Rule::identifier => e.variants.push(String::from(span.as_str())),
        _ => {}
    };

    // for pair in input.clone().into_inner() {
    //     let span = pair.clone().into_span();
    //     println!("{:?}", pair);
    //     match pair.as_rule() {
    //         //Rule::indetifier => e.name = String::from(span.as_str()),
    //         Rule::enum_type_body_auto => e.name = span.as_str().to_owned(),
    //         Rule::enum_type_body_assign => {
    //             // e.properties.push(parse_enum_property(&pair, last_value));
    //             // last_value = e.properties.last().unwrap().value;
    //             println!("{:?}", pair);
    //         }
    //         //Rule::identifier => e.variants.push(String::from(span.as_str())),
    //         _ => {}
    //     };
    // }

    e
}

fn parse_enum(input: &Pair) -> EnumExpression {
    let span = input.clone().into_span();

    let mut e = EnumExpression {
        name: String::from(""),
        properties: vec![],
        location: Location(span.start(), span.end()),
    };
    let mut last_value = -1;
    for pair in input.clone().into_inner() {
        let span = pair.clone().into_span();

        match pair.as_rule() {
            Rule::type_name => e.name = String::from(span.as_str()),
            Rule::enum_type_body_auto | Rule::enum_type_body_assign => {
                e.properties.push(parse_enum_property(&pair, last_value));
                last_value = e.properties.last().unwrap().value;
            }
            //Rule::identifier => e.variants.push(String::from(span.as_str())),
            _ => {}
        };
    }

    e
}

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
        "i64" => Builtin::Int64,
        "i32" => Builtin::Int32,
        "i16" => Builtin::Int16,
        "i8" => Builtin::Int8,
        "u64" => Builtin::Uint64,
        "u32" => Builtin::Uint32,
        "u16" => Builtin::Uint16,
        "u8" => Builtin::Uint8,
        "f32" => Builtin::Float,
        "f64" => Builtin::Double,
        "string" => Builtin::String,
        "bytes" => Builtin::Bytes,
        "bool" => Builtin::Bool,
        "object" => Builtin::Object,
        "date" => Builtin::Date,
        "any" => Builtin::Any,
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

fn parse_array_type(input: &Pair) -> Type {
    let span = input.clone().into_span();
    let mut exp = ArrayExpression {
        location: Location(span.start(), span.end()),
        value: Box::new(TypeExpression::Required(Type::Builtin(Builtin::Void))),
    };

    for pair in input.clone().into_inner() {
        //println!("{:?}", pair);
        exp.value = Box::new(parse_all_type_exp(&pair));
    }

    Type::Array(exp)
}

fn parse_all_type_exp(input: &Pair) -> TypeExpression {
    //println!("{:?}", input);

    match input.as_rule() {
        Rule::generic_type
        | Rule::type_builtins
        | Rule::type_name
        | Rule::repeated_type
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
        // Rule::repeated_type => {
        //     let pair = input.clone().into_inner().next().unwrap();
        //     TypeExpression::Repeated(parse_all_type(&pair))
        // }
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
        Rule::repeated_type => parse_array_type(&input),
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
            // Rule::type_builtins | Rule::type_name | Rule::anonymous_record_type => {
            //     prop.value = TypeExpression::Required(parse_all_type(&pair));
            // }
            // Rule::optional_type => {
            //     let pair = pair.clone().into_inner().next().unwrap();
            //     prop.value = TypeExpression::Optional(parse_all_type(&pair));
            // }
            // Rule::repeated_type => {
            //     let pair = pair.clone().into_inner().next().unwrap();
            //     prop.value = TypeExpression::Repeated(parse_all_type(&pair));
            // }
            _ => {
                prop.value = parse_all_type_exp(&pair);
            }
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
        "head" => HttpMethod::Head,
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

// fn parse_http_endpoint_returns(input: &Pair) -> Vec<HttpEndpointReturnsExpression> {
//     let span = input.clone().into_span();

//     let mut returns = HttpEndpointReturnsExpression {
//         name: String::from(""),
//         location: Location(span.start(), span.end()),
//         value: TypeExpression::Required(Type::Builtin(Builtin::Void)),
//     };

//     let mut output = vec![];

//     for pair in input.clone().into_inner() {
//         let span = pair.clone().into_span();
//         match pair.as_rule() {
//             Rule::http_status_code => returns.name = String::from(span.as_str()),
//             Rule::generic_type | Rule::anonymous_record_type => {
//                 returns.value = parse_all_type_exp(&pair);
//                 output.push(returns.clone());
//             }
//             _ => {}
//         };
//     }

//     output
// }

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
                let inner = pair.clone().into_inner().next().unwrap();
                endpoint
                    .properties
                    .push(HttpEndpointPropertyExpression::Returns(parse_all_type_exp(
                        &inner,
                    )));
            }
            Rule::http_endpoint_body => {
                let inner = pair.clone().into_inner().next().unwrap();

                endpoint
                    .properties
                    .push(HttpEndpointPropertyExpression::Body(parse_all_type_exp(
                        &inner,
                    )));
            }
            Rule::http_endpoint_description => {
                endpoint
                    .properties
                    .push(HttpEndpointPropertyExpression::Description(
                        span.as_str().trim().to_string(),
                    ))
            }
            Rule::http_endpoint_query => {
                let inner = pair.clone().into_inner().next().unwrap();

                endpoint
                    .properties
                    .push(HttpEndpointPropertyExpression::Query(parse_all_type_exp(
                        &inner,
                    )));
            }
            Rule::http_endpoint_auth => {
                let inner = pair.clone().into_inner().next().unwrap();

                let auth = match inner.clone().into_span().as_str() {
                    "token" => HttpEndpointAuthType::Token("bearer".to_owned()),
                    "simple" => HttpEndpointAuthType::Simple,
                    s => panic!(format!("invalid auth type {}", s)),
                };

                endpoint
                    .properties
                    .push(HttpEndpointPropertyExpression::Auth(auth))
            }
            Rule::http_endpoint_headers => {
                let inner = pair.clone().into_inner();
                let span = pair.clone().into_span();
                let mut header = HttpEndpointHeadersExpression {
                    value: vec![],
                    location: Location(span.start(), span.end()),
                };
                for p in inner {
                    let span = p.clone().into_span().as_str().to_owned();
                    match p.as_rule() {
                        Rule::http_endpoint_headers_header_required => {
                            header.value.push(HttpEndpointHeaderExpression {
                                name: span,
                                value: TypeExpression::Required(Type::Builtin(Builtin::String)),
                            });
                        }
                        Rule::http_endpoint_headers_header_optional => {
                            header.value.push(HttpEndpointHeaderExpression {
                                name: span,
                                value: TypeExpression::Optional(Type::Builtin(Builtin::String)),
                            });
                        }
                        _ => {}
                    };
                }
                endpoint
                    .properties
                    .push(HttpEndpointPropertyExpression::Headers(header))
            }
            _ => {}
        };
    }

    endpoint
}

pub fn build_ast(input: &str) -> Result<ModuleExpression> {
    build_ast_path(input, "")
}

pub fn build_ast_path<T: AsRef<Path>>(input: &str, path: T) -> Result<ModuleExpression> {
    let pairs = tokenize(input)?;
    let mut exps: Vec<Expression> = vec![];

    for pair in pairs {
        let exp = match pair.as_rule() {
            Rule::generic_record_type => {
                Expression::GenericRecord(parse_generic_record_type(&pair))
            }
            Rule::record_type => Expression::Record(parse_record(&pair)),
            Rule::import => Expression::Import(parse_import(&pair)),
            Rule::enum_type => Expression::Enum(parse_enum(&pair)),
            Rule::http_endpoint => Expression::HttpEndpoint(parse_http_endpoint(&pair)),
            _ => Expression::Invalid,
        };

        if exp == Expression::Invalid {
            println!("invalid {:?}", pair);
            continue;
        }
        exps.push(exp)
    }

    Ok(ModuleExpression {
        body: exps,
        imports: vec![],
        path: path.as_ref().to_path_buf(),
    })
}
