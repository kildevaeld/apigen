use std::path::PathBuf;
use std::string::String;

#[serde(tag = "type", content = "value")]
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum Type {
    Builtin(Builtin),
    User(String),
    Generic(GenericExpression),
    Anonymous(AnonymousRecordExpression),
}

#[derive(PartialEq, Debug)]
pub enum UserType<'a> {
    Record(&'a RecordExpression),
    GenericRecord(&'a GenericRecordExpression),
    Enum(&'a EnumExpression),
}

impl<'a> UserType<'a> {
    pub fn name(&self) -> String {
        let name = match self {
            UserType::Enum(e) => &e.name,
            UserType::GenericRecord(e) => &e.name,
            UserType::Record(e) => &e.name,
        };
        name.to_owned()
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum Builtin {
    Int8,
    Int16,
    Int32,
    Int64,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Float,
    Double,
    String,
    Bytes,
    Bool,
    Map,
    Date,
    Void,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Location(pub usize, pub usize);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModuleExpression {
    pub path: PathBuf,
    pub imports: Vec<ModuleExpression>,
    pub body: Vec<Expression>,
}

impl ModuleExpression {
    pub fn name(&self) -> String {
        let mut path: PathBuf = self.path.clone();
        path.set_extension("");
        path.file_name().unwrap().to_str().unwrap().to_owned()
    }

    fn build<'a>(&'a self, v: &'a Vec<Expression>) -> Vec<UserType<'a>> {
        v.into_iter()
            .filter_map(|m| match m {
                Expression::Enum(e) => Some(UserType::Enum(&e)),
                Expression::GenericRecord(e) => Some(UserType::GenericRecord(&e)),
                Expression::Record(e) => Some(UserType::Record(&e)),
                _ => None,
            })
            .collect()
    }

    pub fn local_scope<'a>(&'a self) -> Vec<UserType<'a>> {
        self.build(&self.body)
    }

    pub fn imported_scope<'a>(&'a self) -> Vec<UserType<'a>> {
        let mut out = vec![];

        for i in &self.imports {
            let mut v = self.build(&i.body);
            out.append(&mut v);
        }
        out
    }

    pub fn scope<'a>(&'a self) -> Vec<UserType<'a>> {
        let mut out = vec![];
        let mut loc = self.local_scope();
        let mut imp = self.imported_scope();

        out.append(&mut loc);
        out.append(&mut imp);

        out
    }
}

impl IntoIterator for ModuleExpression {
    type Item = Expression;
    type IntoIter = ::std::vec::IntoIter<Expression>;

    fn into_iter(self) -> Self::IntoIter {
        self.body.into_iter()
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct ImportExpression {
    pub path: String,
    pub location: Location,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct RecordExpression {
    pub name: String,
    pub properties: Vec<RecordPropertyExpression>,
    pub location: Location,
    pub annotations: Vec<AnnotationExpression>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct RecordPropertyExpression {
    pub name: String,
    pub value: TypeExpression,
    pub location: Location,
}

#[serde(tag = "type", content = "value")]
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum TypeExpression {
    Required(Type),
    Optional(Type),
    Repeated(Type),
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct GenericExpression {
    pub name: String,
    pub types: Vec<TypeExpression>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct GenericRecordExpression {
    pub name: String,
    pub type_names: Vec<String>,
    pub properties: Vec<RecordPropertyExpression>,
    pub location: Location,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct AnonymousRecordExpression {
    pub properties: Vec<RecordPropertyExpression>,
    pub location: Location,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct EnumExpression {
    pub name: String,
    pub variants: Vec<String>,
    pub location: Location,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct AnnotationExpression {
    pub name: String,
    pub value: JSONExpression,
    pub location: Location,
}

#[serde(tag = "type", content = "value")]
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum JSONExpression {
    Array,
    Object {
        location: Location,
        properties: Vec<ObjectPropertyExpression>,
    },
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct ObjectPropertyExpression {
    pub name: String,
    pub value: JSONExpression,
    pub location: Location,
}

// Endpoint
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum HttpMethod {
    Get,
    Put,
    Post,
    Delete,
    Patch,
}

#[serde(tag = "type", content = "value")]
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum HttpQuery {
    Anonymous(AnonymousRecordExpression),
    Record(String),
    Generic(String, Vec<String>),
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct HttpEndpointExpression {
    pub method: HttpMethod,
    pub path: Vec<HttpEndpointPathExpression>,
    pub properties: Vec<HttpEndpointPropertyExpression>,
    pub location: Location,
}

impl HttpEndpointExpression {
    pub fn name(&self) -> String {
        let s = (&self.path)
            .into_iter()
            .map(|m| match m {
                HttpEndpointPathExpression::Segment(s) => s.clone(),
                HttpEndpointPathExpression::Param(s) => s.clone(),
            })
            .collect::<Vec<String>>()
            .join("_");

        format!("{:?}_{}", self.method, s).to_lowercase()
    }
}

#[serde(tag = "type", content = "value")]
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum HttpEndpointPathExpression {
    Segment(String),
    Param(String),
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]

pub enum HttpEndpointAuthType {
    Token,
    Simple,
}

#[serde(tag = "type", content = "value")]
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum HttpEndpointPropertyExpression {
    Body(TypeExpression),
    Query(TypeExpression),
    Description(String),
    Returns(TypeExpression),
    Auth(HttpEndpointAuthType),
}

#[serde(tag = "type")]
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum Expression {
    Invalid,
    #[serde(rename = "Record")]
    Record(RecordExpression),
    #[serde(rename = "Import")]
    Import(ImportExpression),
    #[serde(rename = "Enum")]
    Enum(EnumExpression),
    #[serde(rename = "GenericRecord")]
    GenericRecord(GenericRecordExpression),
    #[serde(rename = "HttpEndpoint")]
    HttpEndpoint(HttpEndpointExpression),
}
