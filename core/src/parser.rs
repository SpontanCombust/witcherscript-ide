peg::parser! {
    pub grammar parser() for str {
        use crate::lexing::*;
        use crate::ast::classes::*;
        use crate::ast::conditionals::*;
        use crate::ast::enums::*;
        use crate::ast::expressions::*;
        use crate::ast::functions::*;
        use crate::ast::literals::*;
        use crate::ast::loops::*;
        use crate::ast::module::*;
        use crate::ast::nop::*;
        use crate::ast::states::*;
        use crate::ast::structs::*;
        use crate::ast::vars::*;

        use peg::ParseLiteral;

    

        // STATEMENTS =============================================================================
        
        // MODULE =================================

        pub rule module() -> ModuleBody
            = _ v:spanned(<module_stmt()>) ** _ _ {v}

        rule module_stmt() -> ModuleStatement
            = f:func_decl() { ModuleStatement::Function(f) }
            / c:class_decl() { ModuleStatement::Class(c) }
            / s:state_decl() { ModuleStatement::State(s) }
            / s:struct_decl() { ModuleStatement::Struct(s) }
            / e:enum_decl() { ModuleStatement::Enum(e) }
            / nop()



        // ENUM DECLARATION =======================

        pub rule enum_decl() -> EnumDeclaration
            = _ "enum" _ name:spanned(<identifier()>) 
            _ "{" _ body:spanned(<enum_body()>) _ "}" _ {
                EnumDeclaration {
                    name,
                    body
                }
            }

        rule enum_body() -> EnumBody
            = comma_trailing(<spanned(<enum_decl_value()>)>)

        rule enum_decl_value() -> EnumDeclarationValue
            = name:spanned(<identifier()>) _ int_value:("=" _ i:spanned(<literal_int()>) {i})? {
                EnumDeclarationValue { 
                    name, 
                    int_value 
                }
            }



        // STRUCT DECLARATION =====================

        pub rule struct_decl() -> StructDeclaration
            = _ imported:imported() _ "struct" _ name:spanned(<identifier()>)
            _ "{" _ body:spanned(<struct_body()>) _ "}" _ {
                StructDeclaration { 
                    imported, 
                    name, 
                    body
                }
            }

        rule struct_body() -> StructBody
            = v:spanned(<struct_stmt()>) ** _ {v}

        rule struct_stmt() -> StructStatement
            = struct_member_var_decl_stmt()
            / struct_member_default_val_stmt()
            / struct_member_hint_stmt()

        rule struct_member_var_decl_stmt() -> StructStatement
            = v:member_var_decl() {
                StructStatement::Var(v)
            }

        rule struct_member_default_val_stmt() -> StructStatement
            = dv:member_default_val() { 
                StructStatement::Default(dv)
            }

        rule struct_member_hint_stmt() -> StructStatement
            = h:member_hint() { 
                StructStatement::Hint(h)
            }



        // STATE DECLARATION ======================

        pub rule state_decl() -> StateDeclaration
            =  _ imported:imported() _ specifiers:state_specifiers_bitmask() 
            _ "state" _ name:spanned(<identifier()>) 
            _ "in" _ parent_class:spanned(<identifier()>) 
            _ base_state:class_base()? 
            _ "{" _ body:spanned(<class_body()>) _ "}" _ {
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
            = _ imported:imported() _ specifiers:class_specifiers_bitmask() 
            _ "class" _ name:spanned(<identifier()>) _ base_class:class_base()? 
            _ "{" _ body:spanned(<class_body()>) _ "}" _ {
                ClassDeclaration { 
                    imported, 
                    specifiers, 
                    name, 
                    base_class, 
                    body
                }
            }

        rule class_base() -> Spanned<Identifier>
            = "extends" _ b:spanned(<identifier()>) { b }

        rule class_specifiers_bitmask() -> ClassSpecifiers
            = bitmask(<class_specifiers()>)

        rule class_specifiers() -> ClassSpecifiers
            = "abstract" { ClassSpecifiers::Abstract }
            / "statemachine" { ClassSpecifiers::Statemachine }



        // CLASS BODY =============================

        rule class_body() -> ClassBody
        = v:spanned(<class_stmt()>) ** _ {v}

        pub rule class_stmt() -> ClassStatement
            = class_member_var_decl_stmt()
            / class_member_default_val_stmt()
            / class_member_hint_stmt()
            / class_autobind_stmt()
            / class_method_decl_stmt()
            / nop()

        rule class_member_var_decl_stmt() -> ClassStatement
            = v:member_var_decl() { 
                ClassStatement::Var(v)
            }

        rule class_member_default_val_stmt() -> ClassStatement
            = dv:member_default_val() { 
                ClassStatement::Default(dv)
            }

        rule class_member_hint_stmt() -> ClassStatement
            = h:member_hint() { 
                ClassStatement::Hint(h)
            }

        rule class_autobind_stmt() -> ClassStatement
            = access_modifier:spanned(<access_modifier()>)? _ optional:present("optional")  
            _ "autobind" _ name:spanned(<identifier()>) _ autobind_type:spanned(<type_annot()>) 
            _ "=" _ value:spanned(<class_autobind_value()>) _ ";" { 
                ClassStatement::Autobind(ClassAutobind { 
                    access_modifier, 
                    optional, 
                    name, 
                    autobind_type, 
                    value 
                })
            }

        rule class_autobind_value() -> ClassAutobindValue
            = "single" { ClassAutobindValue::Single }
            / s:literal_string() { ClassAutobindValue::Concrete(s) }

        rule class_method_decl_stmt() -> ClassStatement
            = f:func_decl() {
                ClassStatement::Method(f)
            }
            

        rule member_default_val() -> MemberDefaultValue
            = "default" _ member:spanned(<identifier()>) _ "=" _ value:expr() _ ";" {
                MemberDefaultValue { 
                    member, 
                    value 
                }
            }

        rule member_hint() -> MemberHint
            = "hint" _ member:spanned(<identifier()>) _ "=" _ value:spanned(<literal_string()>) _ ";" {
                MemberHint { 
                    member, 
                    value 
                }
            }



        // FUNCTION DECLARATION ===================

        pub rule func_decl() -> FunctionDeclaration
            = _ imported:imported() _ access_modifier:spanned(<access_modifier()>)? 
            _ specifiers:func_specifiers() _ speciality:func_speciality()
            _ name:spanned(<identifier()>) _ "(" _ params:func_parameters() _ ")" 
            _ return_type:spanned(<type_annot()>)? _ body:func_definition() _ {
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
        
        rule func_definition() -> Option<Spanned<FunctionBody>>
            = "{" _ b:spanned(<func_body()>) _ "}" { Some(b) }
            / ";" { None }

        rule func_parameters() -> Vec<FunctionParameter>
            = groups:comma(<func_parameter_group()>) {
                groups.into_iter().flatten().collect()
            }

        rule func_parameter_group() -> Vec<FunctionParameter>
            = is_optional:present("optional") _ is_output:present("out") _ idents:ident_list() _ param_type:spanned(<type_annot()>) {
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

        rule func_speciality() -> Option<Spanned<FunctionSpeciality>>
            = L:p() fs:_func_speciality() R:p() { fs.map(|v| Spanned::new(v, Span::new(L, R))) }

        rule _func_speciality() -> Option<FunctionSpeciality>
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
            = v:spanned(<func_stmt()>) ** _ {v}

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
                FunctionStatement::Var(v)
            }

        rule for_stmt() -> FunctionStatement
            = "for" _ "(" _ init_expr:expr()? _ ";" _ condition:expr()? _ ";" _ iter_expr:expr()? _ ")" _ body:spanned(<func_stmt()>) {
                FunctionStatement::For(ForLoop { 
                    init_expr, 
                    condition, 
                    iter_expr, 
                    body: Box::new(body) 
                })
            }

        rule while_stmt() -> FunctionStatement
            = "while" _ "(" _ condition:expr() _ ")" _ body:spanned(<func_stmt()>) {
                FunctionStatement::While(WhileLoop { 
                    condition, 
                    body: Box::new(body) 
                })
            }

        rule do_while_stmt() -> FunctionStatement
            = "do" _ body:spanned(<func_stmt()>) _ "while" _ "(" _ condition:expr() _ ")" _ ";" {
                FunctionStatement::DoWhile(DoWhileLoop { 
                    condition, 
                    body: Box::new(body) 
                })
            }

        rule if_stmt() -> FunctionStatement
            = "if" _ "(" _ condition:expr() _ ")" _ body:spanned(<func_stmt()>) _ else_body:else_stmt()? {
                FunctionStatement::If(IfConditional { 
                    condition, 
                    body: Box::new(body), 
                    else_body
                })
            }

        rule else_stmt() -> Box<Spanned<FunctionStatement>>
            = "else" _ else_body:spanned(<func_stmt()>) { 
                Box::new(else_body)
            }

        rule switch_stmt() -> FunctionStatement
            = "switch" _ "(" _ matched_expr:expr() _ ")" _ "{" _ cases:spanned(<switch_case_list()>) _ default:switch_default()? _ "}" {
                FunctionStatement::Switch(SwitchConditional { 
                    matched_expr, 
                    cases,
                    default
                })
            }

        rule switch_case_list() -> Vec<Spanned<SwitchConditionalCase>>
            = spanned(<switch_case()>) ** _

        rule switch_case() -> SwitchConditionalCase
            = "case" _ value:expr() _ ":" _ body:spanned(<func_body()>) {
                SwitchConditionalCase { value, body }
            }
        
        rule switch_default() -> Spanned<FunctionBody>
            = "default" _ ":" _ body:spanned(<func_body()>) {
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
            = "{" _ b:spanned(<func_body()>) _ "}" { 
                FunctionStatement::Scope(b) 
            }

        rule expr_stmt() -> FunctionStatement
            = e:expr() _ ";" { 
                FunctionStatement::Expr(e) 
            }
        


        // VAR DECLARATION ========================

        rule member_var_decl() -> MemberVarDeclaration
            = imported:imported() _ access_modifier:spanned(<access_modifier()>)? _ specifiers:var_specifier_bitmask()
            _ "var" _ idents:ident_list() _ var_type:spanned(<type_annot()>) _ ";" {
                MemberVarDeclaration { 
                    imported: imported, 
                    access_modifier, 
                    specifiers, 
                    names: idents, 
                    var_type,
                }
            }

        rule var_decl() -> VarDeclaration
            = "var" _ idents:ident_list() _ var_type:spanned(<type_annot()>) _ init_value:("=" _ v:expr() {v})? _ ";" {
                VarDeclaration {
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
            = ":" _ n:spanned(<identifier()>) _ g:("<" _ g:spanned(<identifier()>) _ ">" {g})? {
                TypeAnnotation { name: n, generic_argument: g }
            }

        rule ident_list() -> Vec<Spanned<Identifier>> 
            = comma_least_one(<spanned(<identifier()>)>)

        rule nop<T: From<Nop>>() -> T
            = ";" { Nop.into() }

        rule imported() -> bool
            = present("import")

        rule access_modifier() -> AccessModifier
            = "private" { AccessModifier::Private }
            / "protected" { AccessModifier::Protected }
            / "public" { AccessModifier::Public }



        // EXPRESSIONS ===============================================================================

        // precedence based on C++'s operator precedence
        // https://en.cppreference.com/w/cpp/language/operator_precedence
        pub rule expr() -> Box<Spanned<Expression>> = precedence!{
            lh:@ _ op:assignment_operator() _ rh:(@) {
                let span = Span::new(lh.span.begin, rh.span.end);
                Box::new(Spanned::new(Expression::AssignmentOperation(lh, op, rh), span)) 
            }
            condition:@ _ "?" _ expr_if_true:expr() _ ":" _ expr_if_false:(@) {
                let span = Span::new(condition.span.begin, expr_if_false.span.end);
                Box::new(Spanned::new(Expression::TernaryConditional { condition, expr_if_true, expr_if_false }, span))
            }
            --
            lh:(@) _ "||" _ rh:@ {
                let span = Span::new(lh.span.begin, rh.span.end);
                Box::new(Spanned::new(Expression::BinaryOperation(lh, BinaryOperator::Or, rh), span))
            }
            lh:(@) _ "&&" _ rh:@ {
                let span = Span::new(lh.span.begin, rh.span.end);
                Box::new(Spanned::new(Expression::BinaryOperation(lh, BinaryOperator::And, rh), span))
            }
            --
            lh:(@) _ "|" _ rh:@ {
                let span = Span::new(lh.span.begin, rh.span.end);
                Box::new(Spanned::new(Expression::BinaryOperation(lh, BinaryOperator::BitOr, rh), span))
            }
            lh:(@) _ "&" _ rh:@ {
                let span = Span::new(lh.span.begin, rh.span.end);
                Box::new(Spanned::new(Expression::BinaryOperation(lh, BinaryOperator::BitAnd, rh), span))
            }
            --
            lh:(@) _ "!=" _ rh:@ {
                let span = Span::new(lh.span.begin, rh.span.end);
                Box::new(Spanned::new(Expression::BinaryOperation(lh, BinaryOperator::NotEqual, rh), span))
            }
            lh:(@) _ "==" _ rh:@ {
                let span = Span::new(lh.span.begin, rh.span.end);
                Box::new(Spanned::new(Expression::BinaryOperation(lh, BinaryOperator::Equal, rh), span))
            }
            --
            lh:(@) _ ">=" _ rh:@ {
                let span = Span::new(lh.span.begin, rh.span.end);
                Box::new(Spanned::new(Expression::BinaryOperation(lh, BinaryOperator::GreaterOrEqual, rh), span))
            }
            lh:(@) _ ">" _ rh:@ {
                let span = Span::new(lh.span.begin, rh.span.end);
                Box::new(Spanned::new(Expression::BinaryOperation(lh, BinaryOperator::Greater, rh), span))
            }
            lh:(@) _ "<=" _ rh:@ {
                let span = Span::new(lh.span.begin, rh.span.end);
                Box::new(Spanned::new(Expression::BinaryOperation(lh, BinaryOperator::LessOrEqual, rh), span))
            }
            lh:(@) _ "<" _ rh:@ {
                let span = Span::new(lh.span.begin, rh.span.end);
                Box::new(Spanned::new(Expression::BinaryOperation(lh, BinaryOperator::Less, rh), span))
            }
            --
            lh:(@) _ "-" _ rh:@ {
                let span = Span::new(lh.span.begin, rh.span.end);
                Box::new(Spanned::new(Expression::BinaryOperation(lh, BinaryOperator::Sub, rh), span))
            }
            lh:(@) _ "+" _ rh:@ {
                let span = Span::new(lh.span.begin, rh.span.end);
                Box::new(Spanned::new(Expression::BinaryOperation(lh, BinaryOperator::Add, rh), span))
            }
            --
            lh:(@) _ "%" _ rh:@ {
                let span = Span::new(lh.span.begin, rh.span.end);
                Box::new(Spanned::new(Expression::BinaryOperation(lh, BinaryOperator::Modulo, rh), span))
            }
            lh:(@) _ "/" _ rh:@ {
                let span = Span::new(lh.span.begin, rh.span.end);
                Box::new(Spanned::new(Expression::BinaryOperation(lh, BinaryOperator::Div, rh), span))
            }
            lh:(@) _ "*" _ rh:@ {
                let span = Span::new(lh.span.begin, rh.span.end);
                Box::new(Spanned::new(Expression::BinaryOperation(lh, BinaryOperator::Multip, rh), span))
            }
            --
            L:p() "new" _ class:spanned(<identifier()>) _ "in" _ lifetime_object:(@) {
                let span = Span::new(L, lifetime_object.span.end);
                Box::new(Spanned::new(Expression::Instantiation { class, lifetime_object }, span))
            }
            L:p() op:unary_operator() expr:(@) {
                let span = Span::new(L, expr.span.end);
                Box::new(Spanned::new(Expression::UnaryOperation(op, expr), span))
            }
            L:p() "(" _ id:spanned(<identifier()>) _ ")" _ expr:(@) {
                let span = Span::new(L, expr.span.end);
                Box::new(Spanned::new(Expression::TypeCast { target_type: id, expr }, span))
            }
            --
            expr:(@) _ "." _ func:spanned(<identifier()>) "(" _ args:opt_expr_list() _ ")" R:p() {
                let span = Span::new(expr.span.begin, R);
                Box::new(Spanned::new(Expression::MethodCall { expr, func, args }, span))
            }
            expr:(@) _ "." _ member:spanned(<identifier()>) R:p() {
                let span = Span::new(expr.span.begin, R);
                Box::new(Spanned::new(Expression::MemberAccess { expr, member }, span))
            }
            expr:(@) "[" _ index:expr() _ "]" R:p() {
                let span = Span::new(expr.span.begin, R);
                Box::new(Spanned::new(Expression::ArrayAccess { expr, index }, span))
            }
            L:p() func:spanned(<identifier()>) "(" _ args:opt_expr_list() _ ")" R:p() { 
                let span = Span::new(L, R);
                Box::new(Spanned::new(Expression::FunctionCall { func, args }, span))
            }
            --
            L:p() "this" R:p() {
                let span = Span::new(L, R);
                Box::new(Spanned::new(Expression::This, span))
            }
            L:p() "super" R:p() {
                let span = Span::new(L, R);
                Box::new(Spanned::new(Expression::Super, span))
            }
            L:p() "parent" R:p() {
                let span = Span::new(L, R);
                Box::new(Spanned::new(Expression::Parent, span))
            }
            L:p() "virtual_parent" R:p() {
                let span = Span::new(L, R);
                Box::new(Spanned::new(Expression::VirtualParent, span))
            }
            L:p() lit:literal() R:p() {
                let span = Span::new(L, R);
                Box::new(Spanned::new(Expression::Literal(lit), span))
            }
            L:p() id:identifier() R:p() {
                let span = Span::new(L, R);
                Box::new(Spanned::new(Expression::Identifier(id), span)) 
            }
            L:p() "(" _ e:expr() _ ")" R:p() {
                let span = Span::new(L, R);
                Box::new(Spanned::new(Expression::Nested(e), span))
            }
        }

        rule opt_expr_list()-> Vec<Option<Box<Spanned<Expression>>>> = v:comma(<expr()?>) {v}

        rule expr_list()-> Vec<Box<Spanned<Expression>>> = v:comma(<expr()>) {v}


        rule assignment_operator() -> AssignmentOperator
            = "=" { AssignmentOperator::Direct }
            / "+=" { AssignmentOperator::Add }
            / "-=" { AssignmentOperator::Sub }
            / "*=" { AssignmentOperator::Multip }
            / "/=" { AssignmentOperator::Div }
            / "%=" { AssignmentOperator::Modulo } 

        rule unary_operator() -> UnaryOperator
            = "-" { UnaryOperator::Negation }
            / "!" { UnaryOperator::Not }
            / "~" { UnaryOperator::BitNot }

        rule identifier() -> Identifier
            = quiet!{ s:$(['_' | 'a'..='z' | 'A'..='Z']['_' | 'a'..='z' | 'A'..='Z' | '0'..='9']*) {? 
                if let Ok(_) = Keyword::try_from(s) {
                    Err("keyword")
                } else {
                    Ok(Identifier::from(s))
                }
            }}
            / expected!("identifier") //TODO no expected! at this stage; make specific type_identifier(), var_identifier() etc.



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
            = "/*" ([^'*'] / ['*']+ [^'*'|'/'])* ['*']+ "/"

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

        rule p() -> usize = position!()

        // When possible, any rule should only return a data representation of an expression not wrapped in Spanned<T>
        // Then higher order rules can wrap these in spanned(<>) if they want to
        // Exceptions can occur such as the expr() rule that always needs to be wrapped as it is an inherently recursive structure
        // If a rule is highly specific and used only once it may also return Spanned<T> outright
        rule spanned<T>(r: rule<T>) -> Spanned<T>
            = L:p() v:r() R:p() { Spanned::new(v, Span::new(L, R)) }
    }
}

pub use parser::*; // so as to not type parser::parser::