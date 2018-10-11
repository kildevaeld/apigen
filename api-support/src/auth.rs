#[derive(Debug, Clone, PartialEq)]
pub enum Authorization {
    Token(String),
    Simple { user_name: String, password: String },
}
