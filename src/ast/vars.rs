#[derive(Debug, PartialEq, Eq)]
pub struct TypeAnnotation {
    name: String,
    generic_argument: Option<String> // only used for arrays
}