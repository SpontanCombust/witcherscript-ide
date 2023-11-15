#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    Name(String),
    Null
}