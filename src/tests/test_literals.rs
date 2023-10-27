#[allow(unused_imports)]
use crate::{parser, ast::literal::Literal};

#[test]
fn test_invalid() {
    let parser = parser::LiteralParser::new();

    assert!(parser.parse("").is_err());
    assert!(parser.parse("abc").is_err());
    assert!(parser.parse("123a").is_err());
    assert!(parser.parse("\"").is_err());
    assert!(parser.parse("\'").is_err());
    assert!(parser.parse("98.1f32").is_err());
}

#[test]
fn test_int() {
    let parser = parser::LiteralParser::new();

    assert_eq!(parser.parse("123").unwrap(), Literal::Int(123));
    assert_eq!(parser.parse("0123").unwrap(), Literal::Int(123));
    assert_eq!(parser.parse("-92161").unwrap(), Literal::Int(-92161));
    assert_eq!(parser.parse("+2137").unwrap(), Literal::Int(2137));
}

#[test]
fn test_float() {
    let parser = parser::LiteralParser::new();

    assert_eq!(parser.parse("0.0").unwrap(), Literal::Float(0.0));
    assert_eq!(parser.parse("0.f").unwrap(), Literal::Float(0.0));
    assert_eq!(parser.parse("1234.5678").unwrap(), Literal::Float(1234.5678));
    assert_eq!(parser.parse("3.14159265f").unwrap(), Literal::Float(3.14159265));
    assert_eq!(parser.parse("-0.0").unwrap(), Literal::Float(-0.0));
    assert_eq!(parser.parse("-126.10").unwrap(), Literal::Float(-126.1));
    assert_eq!(parser.parse("+126.10").unwrap(), Literal::Float(126.1));
}

#[test]
fn test_bool() {
    let parser = parser::LiteralParser::new();

    assert_eq!(parser.parse("true").unwrap(), Literal::Bool(true));
    assert_eq!(parser.parse("false").unwrap(), Literal::Bool(false));
}

#[test]
fn test_string() {
    let parser = parser::LiteralParser::new();

    assert_eq!(parser.parse(r#""""#).unwrap(), Literal::String("".into()));
    assert_eq!(parser.parse(r#""123""#).unwrap(), Literal::String("123".into()));
    assert_eq!(parser.parse(r#""3.14159265f""#).unwrap(), Literal::String("3.14159265f".into()));
    assert_eq!(parser.parse(r#""true""#).unwrap(), Literal::String("true".into()));
    assert_eq!(parser.parse(r#""abc def""#).unwrap(), Literal::String("abc def".into()));
    assert_eq!(parser.parse(r#""On the first day God said \"Hello World!\". And it crashed.""#).unwrap(), Literal::String("On the first day God said \"Hello World!\". And it crashed.".into()));
    assert_eq!(parser.parse(r#""levels\novigrad\novigrad.w2w""#).unwrap(), Literal::String("levels\\novigrad\\novigrad.w2w".into()));
}

#[test]
fn test_name() {
    let parser = parser::LiteralParser::new();

    assert_eq!(parser.parse(r#"''"#).unwrap(), Literal::Name("".into()));
    assert_eq!(parser.parse(r#"'Novigraadan sword 1'"#).unwrap(), Literal::Name("Novigraadan sword 1".into()));
    assert_eq!(parser.parse(r#"'mq2001_journal_2b'"#).unwrap(), Literal::Name("mq2001_journal_2b".into()));
    assert_eq!(parser.parse(r#"'man_geralt_sword_attack_fast_7_lp_40ms'"#).unwrap(), Literal::Name("man_geralt_sword_attack_fast_7_lp_40ms".into()));
    assert_eq!(parser.parse(r#"'name with spaces and \' escape'"#).unwrap(), Literal::Name("name with spaces and ' escape".into()));
}