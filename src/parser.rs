peg::parser! {
    pub grammar parser() for str {
        use crate::ast::literal::*;
        use crate::ast::operators::*;
        use crate::ast::expression::*;
        use crate::ast::identifier::*;

        use std::rc::Rc;
    

        // EXPRESSIONS ===============================================================================

        // precedence based on C++'s operator precedence
        // https://en.cppreference.com/w/cpp/language/operator_precedence
        pub rule expr() -> Rc<Expression> = precedence!{
            lh:@ _ op:assignment_operator() _ rh:(@) {
                Rc::new(Expression::AssignmentOperation(lh, op, rh))
            }
            condition:@ _ "?" _ expr_if_true:expr() _ ":" _ expr_if_false:(@) {
                Rc::new(Expression::TernaryConditional { condition, expr_if_true, expr_if_false })
            }
            --
            lh:(@) _ "||" _ rh:@ {
                Rc::new(Expression::BinaryOperation(lh, LogicalBinaryOperator::Or.into(), rh))
            }
            lh:(@) _ "&&" _ rh:@ {
                Rc::new(Expression::BinaryOperation(lh, LogicalBinaryOperator::And.into(), rh))
            }
            --
            lh:(@) _ "|" _ rh:@ {
                Rc::new(Expression::BinaryOperation(lh, ArithmeticBinaryOperator::BitwiseOr.into(), rh))
            }
            lh:(@) _ "&" _ rh:@ {
                Rc::new(Expression::BinaryOperation(lh, ArithmeticBinaryOperator::BitwiseAnd.into(), rh))
            }
            --
            lh:(@) _ "!=" _ rh:@ {
                Rc::new(Expression::BinaryOperation(lh, RelationalBinaryOperator::NotEqual.into(), rh))
            }
            lh:(@) _ "==" _ rh:@ {
                Rc::new(Expression::BinaryOperation(lh, RelationalBinaryOperator::Equal.into(), rh))
            }
            --
            lh:(@) _ ">=" _ rh:@ {
                Rc::new(Expression::BinaryOperation(lh, RelationalBinaryOperator::GreaterOrEqual.into(), rh))
            }
            lh:(@) _ ">" _ rh:@ {
                Rc::new(Expression::BinaryOperation(lh, RelationalBinaryOperator::Greater.into(), rh))
            }
            lh:(@) _ "<=" _ rh:@ {
                Rc::new(Expression::BinaryOperation(lh, RelationalBinaryOperator::LessOrEqual.into(), rh))
            }
            lh:(@) _ "<" _ rh:@ {
                Rc::new(Expression::BinaryOperation(lh, RelationalBinaryOperator::Less.into(), rh))
            }
            --
            lh:(@) _ "-" _ rh:@ {
                Rc::new(Expression::BinaryOperation(lh, ArithmeticBinaryOperator::Sub.into(), rh))
            }
            lh:(@) _ "+" _ rh:@ {
                Rc::new(Expression::BinaryOperation(lh, ArithmeticBinaryOperator::Add.into(), rh))
            }
            --
            lh:(@) _ "%" _ rh:@ {
                Rc::new(Expression::BinaryOperation(lh, ArithmeticBinaryOperator::Modulo.into(), rh))
            }
            lh:(@) _ "/" _ rh:@ {
                Rc::new(Expression::BinaryOperation(lh, ArithmeticBinaryOperator::Div.into(), rh))
            }
            lh:(@) _ "*" _ rh:@ {
                Rc::new(Expression::BinaryOperation(lh, ArithmeticBinaryOperator::Multip.into(), rh))
            }
            --
            "new" _ class:identifier() _ "in" _ lifetime_object:expr() {
                Rc::new(Expression::Instantiation { class, lifetime_object })
            }
            op:unary_operator() expr:@ {
                Rc::new(Expression::UnaryOperation(op, expr))
            }
            "(" _ id:identifier() _ ")" _ expr:(@) { 
                Rc::new(Expression::TypeCast { target_type: id, expr }) 
            }
            --
            expr:(@) _ "." _ func:identifier() "(" _ args:opt_expr_list() _ ")" {
                Rc::new(Expression::MethodCall { expr, func, args })
            }
            expr:(@) _ "." _ member:identifier() {
                Rc::new(Expression::MemberAccess { expr, member })
            }
            expr:(@) "[" _ index:expr() _ "]" { 
                Rc::new(Expression::ArrayAccess { expr, index }) 
            }
            func:identifier() "(" _ args:opt_expr_list() _ ")" { 
                Rc::new(Expression::FunctionCall { func, args }) 
            }
            --
            "this" {
                Rc::new(Expression::This)
            }
            "super" {
                Rc::new(Expression::Super)
            }
            "parent" {
                Rc::new(Expression::Parent)
            }
            "virtual_parent" {
                Rc::new(Expression::VirtualParent)
            }
            lit:literal() { 
                Rc::new(Expression::Literal(lit)) 
            }
            id:identifier() { 
                Rc::new(Expression::Identifier(id)) 
            }
            "(" _ e:expr() _ ")" { 
                Rc::new(Expression::Nested(e)) 
            }
        }

        rule opt_expr_list()-> Vec<Option<Rc<Expression>>> = v:comma(<expr()?>) {v}

        rule expr_list()-> Vec<Rc<Expression>> = v:comma(<expr()>) {v}


        rule assignment_operator() -> AssignmentOperator
            = "=" { AssignmentOperator::Direct }
            / "+=" { AssignmentOperator::Add }
            / "-=" { AssignmentOperator::Sub }
            / "*=" { AssignmentOperator::Multip }
            / "/=" { AssignmentOperator::Div }
            / "%=" { AssignmentOperator::Modulo } 

        rule unary_operator() -> UnaryOperator
            = "-" { UnaryOperator::Negation }
            / "!" { UnaryOperator::LogicalNot }
            / "~" { UnaryOperator::BitwiseNot }

        rule identifier() -> Identifier
            = quiet!{ s:$(['_' | 'a'..='z' | 'A'..='Z']['_' | 'a'..='z' | 'A'..='Z' | '0'..='9']*) { Identifier::from(s) } }
            / expected!("identifier")


        // LITERALS ===============================================================================

        pub rule literal() -> Literal
            = s:literal_string() { Literal::String(s) }
            / n:literal_name() { Literal::Name(n) }
            / f:literal_float() { Literal::Float(f) }
            / i:literal_int() { Literal::Int(i) } 
            / b:literal_bool() { Literal::Bool(b) }
            / literal_null() { Literal::Null }

        rule literal_int() -> i32
            = quiet!{ i:$("-"? ['0'..='9']+) {? i.parse().or(Err("i32")) } }
            / expected!("int literal")

        rule literal_float() -> f32
            = quiet!{ f:$("-"? ['0'..='9']+ "." ['0'..='9']*) "f"? {? f.parse().or(Err("f32")) } } 
            / expected!("float literal")

        rule literal_bool() -> bool
            = "true" { true }
            / "false" { false }
        
        rule string_char() -> char 
            = r#"\""# { '\"' }
            / r#"\'"# { '\'' }
            / !['\"' | '\''] c:[_] { c }
            / expected!("string character")

        rule literal_string() -> String
            = quiet!{ "\"" s:string_char()* "\"" { s.into_iter().collect() } }
            / expected!("string literal")

        rule literal_name() -> String
            = quiet!{ "\'" s:string_char()* "\'" { s.into_iter().collect() } }
            / expected!("name literal")

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