#[allow(unused_imports)]
use crate::{parser::*, ast::literal::Literal};

#[test]
fn test_invalid() {
    assert!(literal("").is_err());
    assert!(literal("abc").is_err());
    assert!(literal("123a").is_err());
    assert!(literal("\"").is_err());
    assert!(literal("\'").is_err());
    assert!(literal("\"abc\"def\"").is_err());
    assert!(literal("98.1f32").is_err());
    assert!(literal("NULLnot").is_err());
    assert!(literal("null").is_err());
}

#[test]
fn test_int() {
    assert_eq!(literal("123").unwrap(), Literal::Int(123));
    assert_eq!(literal("0123").unwrap(), Literal::Int(123));
    assert_eq!(literal("-92161").unwrap(), Literal::Int(-92161));
    assert_eq!(literal("2137").unwrap(), Literal::Int(2137));
}

#[test]
fn test_float() {
    assert_eq!(literal("0.0").unwrap(), Literal::Float(0.0));
    assert_eq!(literal("0.f").unwrap(), Literal::Float(0.0));
    assert_eq!(literal("1234.5678").unwrap(), Literal::Float(1234.5678));
    assert_eq!(literal("3.14159265f").unwrap(), Literal::Float(3.14159265));
    assert_eq!(literal("-0.0").unwrap(), Literal::Float(-0.0));
    assert_eq!(literal("-126.10").unwrap(), Literal::Float(-126.1));
    assert_eq!(literal("126.10").unwrap(), Literal::Float(126.1));
}

#[test]
fn test_bool() {
    assert_eq!(literal("true").unwrap(), Literal::Bool(true));
    assert_eq!(literal("false").unwrap(), Literal::Bool(false));
}

#[test]
fn test_string() {
    assert_eq!(literal(r#""""#).unwrap(), Literal::String("".into()));
    assert_eq!(literal(r#""123""#).unwrap(), Literal::String("123".into()));
    assert_eq!(literal(r#""3.14159265f""#).unwrap(), Literal::String("3.14159265f".into()));
    assert_eq!(literal(r#""true""#).unwrap(), Literal::String("true".into()));
    assert_eq!(literal(r#""abc def""#).unwrap(), Literal::String("abc def".into()));
    assert_eq!(literal(r#""On the first day God said \"Hello World!\". And it crashed.""#).unwrap(), Literal::String(r#"On the first day God said "Hello World!". And it crashed."#.into()));
    assert_eq!(literal(r#""levels\novigrad\novigrad.w2w""#).unwrap(), Literal::String(r#"levels\novigrad\novigrad.w2w"#.into()));
}

#[test]
fn test_name() {
    assert_eq!(literal(r#"''"#).unwrap(), Literal::Name("".into()));
    assert_eq!(literal(r#"'Novigraadan sword 1'"#).unwrap(), Literal::Name("Novigraadan sword 1".into()));
    assert_eq!(literal(r#"'mq2001_journal_2b'"#).unwrap(), Literal::Name("mq2001_journal_2b".into()));
    assert_eq!(literal(r#"'man_geralt_sword_attack_fast_7_lp_40ms'"#).unwrap(), Literal::Name("man_geralt_sword_attack_fast_7_lp_40ms".into()));
    assert_eq!(literal(r#"'name with spaces and \' escape'"#).unwrap(), Literal::Name("name with spaces and ' escape".into()));
}

#[test]
fn test_null() {
    assert_eq!(literal("NULL").unwrap(), Literal::Null);
}