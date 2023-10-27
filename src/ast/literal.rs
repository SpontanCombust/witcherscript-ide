#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    Name(String)
}

pub(crate) fn parse_string_like(s: &str) -> String {
    s[1..s.len()-1] // eliminate surrounding quotes
    .replace(r#"\""#, r#"""#) // eliminate escaped quotes
    .replace(r#"\'"#, r#"'"#)
}