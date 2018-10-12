#[derive(Debug, Clone, PartialEq)]
pub enum Authorization {
    Token(String),
    Simple(String, String),
}
