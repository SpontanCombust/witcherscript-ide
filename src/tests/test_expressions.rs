#[allow(unused_imports)]
use crate::parser::*;

#[test]
fn test_comma() {
    let parser = IdentifierListParser::new();

    assert_eq!(parser.parse("abc, def, ghi,jklmno, pq").unwrap(), vec!["abc", "def", "ghi", "jklmno", "pq"]);
    assert_eq!(parser.parse("a").unwrap(), vec!["a"]);
    assert_eq!(parser.parse("a12,    bc3").unwrap(), vec!["a12", "bc3"]);
    assert_eq!(parser.parse("").unwrap(), Vec::<String>::new());
}

#[test]
fn test_cast() {
    let parser = ExprParser::new();

    // println!("{:?}", parser.parse("(abc)(def)").unwrap());
    println!("{:?}", parser.parse("(int)(arr)").unwrap());
}