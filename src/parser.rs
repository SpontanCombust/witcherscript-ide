

peg::parser! {
    pub grammar parser() for str {
        use crate::ast::literal::*;

        // LITERALS ===============================================================================

        pub rule literal() -> Literal
            = s:literal_string() { Literal::String(s) }
            / n:literal_name() { Literal::Name(n) }
            / f:literal_float() { Literal::Float(f) }
            / i:literal_int() { Literal::Int(i) } 
            / b:literal_bool() { Literal::Bool(b) }
            / literal_null() { Literal::Null }

        rule literal_int() -> i32
            = i:$("-"? ['0'..='9']+) {? 
                i.parse().or(Err("i32")) 
            }

        rule literal_float() -> f32
            = f:$("-"? ['0'..='9']+ "." ['0'..='9']*) "f"? {?
                f.parse().or(Err("f32"))
            }

        rule literal_bool() -> bool
            = "true" { true }
            / "false" { false }
        
        rule string_char() -> char 
            = r#"\""# { '\"' }
            / r#"\'"# { '\'' }
            / !['\"' | '\''] c:[_] { c }

        rule literal_string() -> String
            = "\"" s:string_char()* "\"" { s.into_iter().collect() }

        rule literal_name() -> String
            = "\'" s:string_char()* "\'" { s.into_iter().collect() }

        rule literal_null() -> ()
            = "NULL"


        // WHITESPACE & UTILITIES =================================================================

        rule _() = quiet!{ ([' ' | '\n' | '\t' | '\r'] / multiline_comment() / line_comment())* }

        //TODO testing needed
        rule line_comment()
            = "//" [^ '\n'|'\r']* ['\n'|'\r']*

        rule multiline_comment()
            = "/*" (!"*/" [_])* "*/"

        rule comma<T>(x: rule<T>) -> Vec<T>
            = v:(x() ** (_ "," _)) {v}
    }
}

pub use parser::*; // so as to not type parser::parser::