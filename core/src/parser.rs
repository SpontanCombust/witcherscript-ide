peg::parser! {
    pub grammar parser() for str {
        use crate::ast::classes::*;
        use crate::ast::conditionals::*;
        use crate::ast::enums::*;
        use crate::ast::expressions::*;
        use crate::ast::functions::*;
        use crate::ast::identifier::*;
        use crate::ast::literal::*;
        use crate::ast::loops::*;
        use crate::ast::module::*;
        use crate::ast::nop::*;
        use crate::ast::operators::*;
        use crate::ast::states::*;
        use crate::ast::structs::*;
        use crate::ast::vars::*;

        use peg::ParseLiteral;

    

        // STATEMENTS =============================================================================
        
        // MODULE =================================

        pub rule module() -> ModuleBody
            = v:module_stmt() ** _ {v}

        rule module_stmt() -> ModuleStatement
            = f:func_decl() { ModuleStatement::FunctionDeclaration(f) }
            / c:class_decl() { ModuleStatement::ClassDeclaration(c) }
            / s:state_decl() { ModuleStatement::StateDeclaration(s) }
            / s:struct_decl() { ModuleStatement::StructDeclaration(s) }
            / e:enum_decl() { ModuleStatement::EnumDeclaration(e) }
            / nop()



        // ENUM DECLARATION =======================

        pub rule enum_decl() -> EnumDeclaration
            = "enum" _ name:identifier() _ "{" _ body:enum_body() _ "}" {
                EnumDeclaration {
                    name,
                    body
                }
            }

        rule enum_body() -> EnumBody
            = comma_trailing(<enum_decl_value()>)

        rule enum_decl_value() -> EnumDeclarationValue
            = name:identifier() _ int_value:("=" _ i:literal_int() {i})? {
                EnumDeclarationValue { 
                    name, 
                    int_value 
                }
            }



        // STRUCT DECLARATION =====================

        pub rule struct_decl() -> StructDeclaration
            = imported:imported() _ "struct" _ name:identifier() 
            _ "{" _ body:struct_body() _ "}" {
                StructDeclaration { 
                    imported, 
                    name, 
                    body
                }
            }

        rule struct_body() -> StructBody
            = v:struct_stmt() ** _ {v}

        rule struct_stmt() -> StructStatement
            = struct_member_var_decl_stmt()
            / struct_member_default_val_stmt()
            / struct_member_hint_stmt()

        rule struct_member_var_decl_stmt() -> StructStatement
            = v:var_decl() {
                StructStatement::MemberDeclaration(v)
            }

        rule struct_member_default_val_stmt() -> StructStatement
            = t:member_default_val() { 
                StructStatement::MemberDefaultValue { member: t.0, value: t.1 }
            }

        rule struct_member_hint_stmt() -> StructStatement
            = t:member_hint() { 
                StructStatement::MemberHint { member: t.0, value: t.1 }
            }



        // STATE DECLARATION ======================

        pub rule state_decl() -> StateDeclaration
            = imported:imported() _ specifiers:state_specifiers_bitmask() _ "state" _ name:identifier() 
            _ "in" _ parent_class:identifier() _ base_state:class_base()? _ "{" _ body:class_body() _ "}" {
                StateDeclaration { 
                    imported, 
                    specifiers, 
                    name,
                    parent_class,
                    base_state, 
                    body
                }
            }

        rule state_specifiers_bitmask() -> StateSpecifiers
            = "abstract" { StateSpecifiers::Abstract }
            / { StateSpecifiers::none() }



        // CLASS DECLARATION ======================

        pub rule class_decl() -> ClassDeclaration
            = imported:imported() _ specifiers:class_specifiers_bitmask() _ "class" _ name:identifier() 
            _ base_class:class_base()? _ "{" _ body:class_body() _ "}" {
                ClassDeclaration { 
                    imported, 
                    specifiers, 
                    name, 
                    base_class, 
                    body
                }
            }

        rule class_base() -> Identifier
            = "extends" _ b:identifier() { b }

        rule class_specifiers_bitmask() -> ClassSpecifiers
            = bitmask(<class_specifiers()>)

        rule class_specifiers() -> ClassSpecifiers
            = "abstract" { ClassSpecifiers::Abstract }
            / "statemachine" { ClassSpecifiers::Statemachine }



        // CLASS BODY =============================

        rule class_body() -> ClassBody
            = v:class_stmt() ** _ {v}

        pub rule class_stmt() -> ClassStatement
            = class_member_var_decl_stmt()
            / class_member_default_val_stmt()
            / class_member_hint_stmt()
            / class_autobind_stmt()
            / class_method_decl_stmt()
            / nop()

        rule class_member_var_decl_stmt() -> ClassStatement
            = v:var_decl() { 
                ClassStatement::MemberDeclaration(v)
            }

        rule class_member_default_val_stmt() -> ClassStatement
            = t:member_default_val() { 
                ClassStatement::MemberDefaultValue { member: t.0, value: t.1 }
            }

        rule class_member_hint_stmt() -> ClassStatement
            = t:member_hint() { 
                ClassStatement::MemberHint { member: t.0, value: t.1 }
            }

        rule class_autobind_stmt() -> ClassStatement
            = access_modifier:access_modifier()? _ optional:present("optional")  _ "autobind" 
            _ name:identifier() _ autobind_type:type_annot() _ "=" _ value:class_autobind_value() _ ";" { 
                ClassStatement::Autobind(ClassAutobind { 
                    access_modifier, 
                    optional, 
                    name, 
                    autobind_type, 
                    value 
                })
            }

        rule class_autobind_value() -> Option<String>
            = "single" { None }
            / s:literal_string() { Some(s) }

        rule class_method_decl_stmt() -> ClassStatement
            = f:func_decl() {
                ClassStatement::MethodDeclaration(f)
            }
            

        rule member_default_val() -> (Identifier, LiteralOrIdentifier)
            = "default" _ ident:identifier() _ "=" _ val:literal_or_identifier() _ ";" {
                (ident, val)
            }

        rule member_hint() -> (Identifier, String)
            = "hint" _ ident:identifier() _ "=" _ val:literal_string() _ ";" {
                (ident, val)
            }



        // FUNCTION DECLARATION ===================

        pub rule func_decl() -> FunctionDeclaration
            = imported:imported() _ access_modifier:access_modifier()? _ specifiers:func_specifiers() _ speciality:func_speciality()
            _ name:identifier() _ "(" _ params:func_parameters() _ ")" _ return_type:type_annot()? _ body:func_definition() {
                FunctionDeclaration { 
                    imported, 
                    access_modifier, 
                    specifiers, 
                    speciality, 
                    name, 
                    params, 
                    return_type, 
                    body
                }
            }
        
        rule func_definition() -> Option<FunctionBody>
            = "{" _ b:func_body() _ "}" { Some(b) }
            / ";" { None }

        rule func_parameters() -> Vec<FunctionParameter>
            = groups:comma(<func_parameter_group()>) {
                groups.into_iter().flatten().collect()
            }

        rule func_parameter_group() -> Vec<FunctionParameter>
            = is_optional:present("optional") _ is_output:present("out") _ idents:ident_list() _ param_type:type_annot() {
                let mut params = vec![];
                for ident in idents.into_iter() {
                    params.push(FunctionParameter { 
                        name: ident, 
                        is_optional, 
                        is_output, 
                        param_type: param_type.clone() 
                    });
                }
                params
            }

        rule func_speciality() -> Option<FunctionSpeciality>
            = "entry" _ "function" { Some(FunctionSpeciality::Entry) }
            / "event" { Some(FunctionSpeciality::Event) }
            / "exec" _ "function" { Some(FunctionSpeciality::Exec) }
            / "quest" _ "function" { Some(FunctionSpeciality::Quest) }
            / "timer" _ "function" { Some(FunctionSpeciality::Timer) }
            / "storyscene" _ "function" { Some(FunctionSpeciality::Storyscene) }
            / "function" { None }

        rule func_specifiers_bitmask() -> FunctionSpecifiers
            = bitmask(<func_specifiers()>)

        rule func_specifiers() -> FunctionSpecifiers
            = "latent" { FunctionSpecifiers::Latent }
            / "final" { FunctionSpecifiers::Final }



        // FUNCTION BODY ==========================

        rule func_body() -> FunctionBody
            = v:func_stmt() ** _ {v}

        pub rule func_stmt() -> FunctionStatement
            = func_var_decl_stmt()
            / for_stmt()
            / while_stmt()
            / do_while_stmt()
            / if_stmt()
            / switch_stmt()
            / break_stmt()
            / continue_stmt()
            / return_stmt()
            / delete_stmt()
            / scope_stmt()
            / expr_stmt()
            / nop()

        rule func_var_decl_stmt() -> FunctionStatement
            = v:var_decl() { 
                FunctionStatement::VarDeclaration(v)
            }

        rule for_stmt() -> FunctionStatement
            = "for" _ "(" _ init_expr:expr()? _ ";" _ condition:expr()? _ ";" _ iter_expr:expr()? _ ")" _ body:func_stmt() {
                FunctionStatement::For(ForLoop { 
                    init_expr, 
                    condition, 
                    iter_expr, 
                    body: Box::new(body) 
                })
            }

        rule while_stmt() -> FunctionStatement
            = "while" _ "(" _ condition:expr() _ ")" _ body:func_stmt() {
                FunctionStatement::While(WhileLoop { 
                    condition, 
                    body: Box::new(body) 
                })
            }

        rule do_while_stmt() -> FunctionStatement
            = "do" _ body:func_stmt() _ "while" _ "(" _ condition:expr() _ ")" _ ";" {
                FunctionStatement::DoWhile(DoWhileLoop { 
                    condition, 
                    body: Box::new(body) 
                })
            }

        rule if_stmt() -> FunctionStatement
            = "if" _ "(" _ condition:expr() _ ")" _ body:func_stmt() _ else_body:else_stmt()? {
                FunctionStatement::If(IfConditional { 
                    condition, 
                    body: Box::new(body), 
                    else_body
                })
            }

        rule else_stmt() -> Box<FunctionStatement>
            = "else" _ else_body:func_stmt() { 
                Box::new(else_body)
            }

        rule switch_stmt() -> FunctionStatement
            = "switch" _ "(" _ matched_expr:expr() _ ")" _ "{" _ cases:switch_case() ** _ _ default:switch_default()? _ "}" {
                FunctionStatement::Switch(SwitchConditional { 
                    matched_expr, 
                    cases,
                    default
                })
            }

        rule switch_case() -> SwitchConditionalCase
            = "case" _ value:expr() _ ":" _ body:func_body() {
                SwitchConditionalCase { value, body }
            }
        
        rule switch_default() -> FunctionBody
            = "default" _ ":" _ body:func_body() {
                body
            }

        rule break_stmt() -> FunctionStatement
            = "break" _ ";" {
                FunctionStatement::Break
            }

        rule continue_stmt() -> FunctionStatement
            = "continue" _ ";" {
                FunctionStatement::Continue
            }

        rule return_stmt() -> FunctionStatement
            = "return" _ retval:expr()? _ ";" {
                FunctionStatement::Return(retval)
            }

        rule delete_stmt() -> FunctionStatement
            = "delete" _ val:expr() _ ";" {
                FunctionStatement::Delete(val)
            }

        rule scope_stmt() -> FunctionStatement
            = "{" _ b:func_body() _ "}" { 
                FunctionStatement::Scope(b) 
            }

        rule expr_stmt() -> FunctionStatement
            = e:expr() _ ";" { 
                FunctionStatement::Expr(e) 
            }
        


        // VAR DECLARATION ========================

        rule var_decl() -> VarDeclaration
            = imported:imported() _ access_modifier:access_modifier()? _ specifiers:var_specifier_bitmask()
            _ "var" _ idents:ident_list() _ var_type:type_annot() _ init_value:("=" _ v:expr() {v})? _ ";" {
                VarDeclaration { 
                    imported: imported, 
                    access_modifier, 
                    specifiers, 
                    names: idents, 
                    var_type,
                    init_value
                }
            }
        
        rule var_specifier_bitmask() -> VarSpecifiers
            = bitmask(<var_specifier()>)

        rule var_specifier() -> VarSpecifiers
            = "const" { VarSpecifiers::Const }
            / "editable" { VarSpecifiers::Editable }
            / "inlined" { VarSpecifiers::Inlined }
            / "saved" { VarSpecifiers::Saved }



        // COMMON =================================

        rule type_annot() -> TypeAnnotation
            = ":" _ n:identifier() _ g:("<" _ g:identifier() _ ">" {g})? {
                TypeAnnotation { name: n, generic_argument: g }
            }

        rule ident_list() -> Vec<Identifier> 
            = comma_least_one(<identifier()>)

        rule nop<T: From<Nop>>() -> T
            = ";" { Nop.into() }

        rule imported() -> bool
            = present("import")

        rule access_modifier() -> AccessModifier
            = "private" { AccessModifier::Private }
            / "protected" { AccessModifier::Protected }
            / "public" { AccessModifier::Public }

        rule literal_or_identifier() -> LiteralOrIdentifier
            = lit:literal() { LiteralOrIdentifier::Literal(lit) }
            / id:identifier() { LiteralOrIdentifier::Identifier(id) }



        // EXPRESSIONS ===============================================================================

        // precedence based on C++'s operator precedence
        // https://en.cppreference.com/w/cpp/language/operator_precedence
        pub rule expr() -> Box<Expression> = precedence!{
            lh:@ _ op:assignment_operator() _ rh:(@) {
                Box::new(Expression::AssignmentOperation(lh, op, rh))
            }
            condition:@ _ "?" _ expr_if_true:expr() _ ":" _ expr_if_false:(@) {
                Box::new(Expression::TernaryConditional { condition, expr_if_true, expr_if_false })
            }
            --
            lh:(@) _ "||" _ rh:@ {
                Box::new(Expression::BinaryOperation(lh, LogicalBinaryOperator::Or.into(), rh))
            }
            lh:(@) _ "&&" _ rh:@ {
                Box::new(Expression::BinaryOperation(lh, LogicalBinaryOperator::And.into(), rh))
            }
            --
            lh:(@) _ "|" _ rh:@ {
                Box::new(Expression::BinaryOperation(lh, ArithmeticBinaryOperator::BitwiseOr.into(), rh))
            }
            lh:(@) _ "&" _ rh:@ {
                Box::new(Expression::BinaryOperation(lh, ArithmeticBinaryOperator::BitwiseAnd.into(), rh))
            }
            --
            lh:(@) _ "!=" _ rh:@ {
                Box::new(Expression::BinaryOperation(lh, RelationalBinaryOperator::NotEqual.into(), rh))
            }
            lh:(@) _ "==" _ rh:@ {
                Box::new(Expression::BinaryOperation(lh, RelationalBinaryOperator::Equal.into(), rh))
            }
            --
            lh:(@) _ ">=" _ rh:@ {
                Box::new(Expression::BinaryOperation(lh, RelationalBinaryOperator::GreaterOrEqual.into(), rh))
            }
            lh:(@) _ ">" _ rh:@ {
                Box::new(Expression::BinaryOperation(lh, RelationalBinaryOperator::Greater.into(), rh))
            }
            lh:(@) _ "<=" _ rh:@ {
                Box::new(Expression::BinaryOperation(lh, RelationalBinaryOperator::LessOrEqual.into(), rh))
            }
            lh:(@) _ "<" _ rh:@ {
                Box::new(Expression::BinaryOperation(lh, RelationalBinaryOperator::Less.into(), rh))
            }
            --
            lh:(@) _ "-" _ rh:@ {
                Box::new(Expression::BinaryOperation(lh, ArithmeticBinaryOperator::Sub.into(), rh))
            }
            lh:(@) _ "+" _ rh:@ {
                Box::new(Expression::BinaryOperation(lh, ArithmeticBinaryOperator::Add.into(), rh))
            }
            --
            lh:(@) _ "%" _ rh:@ {
                Box::new(Expression::BinaryOperation(lh, ArithmeticBinaryOperator::Modulo.into(), rh))
            }
            lh:(@) _ "/" _ rh:@ {
                Box::new(Expression::BinaryOperation(lh, ArithmeticBinaryOperator::Div.into(), rh))
            }
            lh:(@) _ "*" _ rh:@ {
                Box::new(Expression::BinaryOperation(lh, ArithmeticBinaryOperator::Multip.into(), rh))
            }
            --
            "new" _ class:identifier() _ "in" _ lifetime_object:expr() {
                Box::new(Expression::Instantiation { class, lifetime_object })
            }
            op:unary_operator() expr:@ {
                Box::new(Expression::UnaryOperation(op, expr))
            }
            "(" _ id:identifier() _ ")" _ expr:(@) { 
                Box::new(Expression::TypeCast { target_type: id, expr }) 
            }
            --
            expr:(@) _ "." _ func:identifier() "(" _ args:opt_expr_list() _ ")" {
                Box::new(Expression::MethodCall { expr, func, args })
            }
            expr:(@) _ "." _ member:identifier() {
                Box::new(Expression::MemberAccess { expr, member })
            }
            expr:(@) "[" _ index:expr() _ "]" { 
                Box::new(Expression::ArrayAccess { expr, index }) 
            }
            func:identifier() "(" _ args:opt_expr_list() _ ")" { 
                Box::new(Expression::FunctionCall { func, args }) 
            }
            --
            "this" {
                Box::new(Expression::This)
            }
            "super" {
                Box::new(Expression::Super)
            }
            "parent" {
                Box::new(Expression::Parent)
            }
            "virtual_parent" {
                Box::new(Expression::VirtualParent)
            }
            lit:literal() { 
                Box::new(Expression::Literal(lit)) 
            }
            id:identifier() { 
                Box::new(Expression::Identifier(id)) 
            }
            "(" _ e:expr() _ ")" { 
                Box::new(Expression::Nested(e)) 
            }
        }

        rule opt_expr_list()-> Vec<Option<Box<Expression>>> = v:comma(<expr()?>) {v}

        rule expr_list()-> Vec<Box<Expression>> = v:comma(<expr()>) {v}


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

        rule comma_trailing<T>(x: rule<T>) -> Vec<T>
            = v:(x() ** (_ "," _)) _ ","? {v}

        rule comma_least_one<T>(x: rule<T>) -> Vec<T>
            = v:(x() ++ (_ "," _)) {v}

        rule present(k: &'static str) -> bool
            = ##parse_string_literal(k) { true }
            / { false }

        rule bitmask<T: Into<u8>, B: From<u8>>(b: rule<T>) -> B
            = v:b() ** _ {
                let mut b = 0u8;
                for val in v {
                    b |= val.into();
                }
                b.into()
            }
    }
}

pub use parser::*; // so as to not type parser::parser::