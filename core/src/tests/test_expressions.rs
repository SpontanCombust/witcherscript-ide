#[allow(unused_imports)]
use crate::{
    ast::expressions::*,
    parser::*
};



#[test]
fn test_keywords() {
    assert_eq!(
        format!("{:?}", expr("this")), 
        r#"Ok(This)"#
    );
    assert_eq!(
        format!("{:?}", expr("super")), 
        r#"Ok(Super)"#
    );
    assert_eq!(
        format!("{:?}", expr("parent")), 
        r#"Ok(Parent)"#
    );
    assert_eq!(
        format!("{:?}", expr("virtual_parent")), 
        r#"Ok(VirtualParent)"#
    );
}

#[test]
fn test_literals() {
    assert_eq!(
        format!("{:?}", expr("126")), 
        r#"Ok(Literal(Int(126)))"#
    );
    assert_eq!(
        format!("{:?}", expr("0")), 
        r#"Ok(Literal(Int(0)))"#
    );
    assert_eq!(
        format!("{:?}", expr("21.37f")), 
        r#"Ok(Literal(Float(21.37)))"#
    );
    assert_eq!(
        format!("{:?}", expr("0.07")), 
        r#"Ok(Literal(Float(0.07)))"#
    );
    assert_eq!(
        format!("{:?}", expr("true")), 
        r#"Ok(Literal(Bool(true)))"#
    );
    assert_eq!(
        format!("{:?}", expr("false")), 
        r#"Ok(Literal(Bool(false)))"#
    );
    assert_eq!(
        format!("{:?}", expr(r#""levels\novigrad\novigrad.w2w""#)), 
        r#"Ok(Literal(String("levels\\novigrad\\novigrad.w2w")))"#
    );
    assert_eq!(
        format!("{:?}", expr(r#"'runeword_4'"#)), 
        r#"Ok(Literal(Name("runeword_4")))"#
    );
    assert_eq!(
        format!("{:?}", expr("NULL")), 
        r#"Ok(Literal(Null))"#
    );
}

#[test]
fn test_identifier() {
    assert_eq!(
        format!("{:?}", expr("thePlayer")), 
        r#"Ok(Identifier("thePlayer"))"#
    );
    assert_eq!(
        format!("{:?}", expr("WALK_DEEP_WATER_LEVEL")), 
        r#"Ok(Identifier("WALK_DEEP_WATER_LEVEL"))"#
    );
}

#[test]
fn test_nesting() {
    assert_eq!(
        format!("{:?}", expr("( EPMT_Mutation6 )")), 
        r#"Ok(Nested(Identifier("EPMT_Mutation6")))"#
    );
}

#[test]
fn test_array_access() {
    assert_eq!(
        format!("{:?}", expr("points[i-1].inVal")), 
        r#"Ok(MemberAccess { expr: ArrayAccess { expr: Identifier("points"), index: BinaryOperation(Identifier("i"), Airthmetic(Sub), Literal(Int(1))) }, member: "inVal" })"#
    );
}

#[test]
fn test_func_call() {
    assert_eq!(
        format!("{:?}", expr("SomeFunc(arg1, 'arg2', 3.0f, Arg4())")), 
        r#"Ok(FunctionCall { func: "SomeFunc", args: [Some(Identifier("arg1")), Some(Literal(Name("arg2"))), Some(Literal(Float(3.0))), Some(FunctionCall { func: "Arg4", args: [None] })] })"#
    );
}

#[test]
fn test_member_access() {
    assert_eq!(
        format!("{:?}", expr("thePlayer.inv.sword")), 
        r#"Ok(MemberAccess { expr: MemberAccess { expr: Identifier("thePlayer"), member: "inv" }, member: "sword" })"#
    );
}

#[test]
fn test_method_call() {
    assert_eq!(
        format!("{:?}", expr("horseManager.GetInventoryComponent()")), 
        r#"Ok(MethodCall { expr: Identifier("horseManager"), func: "GetInventoryComponent", args: [None] })"#
    );
    assert_eq!(
        format!("{:?}", expr("inv.GetItemEnhancementCount(swords[i])")), 
        r#"Ok(MethodCall { expr: Identifier("inv"), func: "GetItemEnhancementCount", args: [Some(ArrayAccess { expr: Identifier("swords"), index: Identifier("i") })] })"#
    );
}

#[test]
fn test_instantiation() {
    assert_eq!(
        format!("{:?}", expr("new W3DamageAction in theGame.damageMgr")), 
        r#"Ok(Instantiation { class: "W3DamageAction", lifetime_object: MemberAccess { expr: Identifier("theGame"), member: "damageMgr" } })"#
    );
}

#[test]
fn test_type_cast() {
    assert_eq!(
        format!("{:?}", expr("(W3PlayerWitcher)thePlayer")), 
        r#"Ok(TypeCast { target_type: "W3PlayerWitcher", expr: Identifier("thePlayer") })"#
    );
}

#[test]
fn test_unary_operators() {
    //TODO test_unary_operators
}

#[test]
fn test_binary_operators() {
    //TODO test_binary_operators
}

#[test]
fn test_assignment_operators() {
    //TODO test_assignment_operators
}

#[test]
fn test_ternary_conditional() {
    assert_eq!(
        format!("{:?}", expr("actor.IsAlive() ? 0.0f : 0.01f")), 
        r#"Ok(TernaryConditional { condition: MethodCall { expr: Identifier("actor"), func: "IsAlive", args: [None] }, expr_if_true: Literal(Float(0.0)), expr_if_false: Literal(Float(0.01)) })"#
    );
}

#[test]
fn test_complex() {
    assert_eq!(
        format!("{:?}", expr(r#"((CMovingPhysicalAgentComponent)((CNewNPC)results[i]).GetMovingAgentComponent()).SetAnimatedMovement( false )"#)), 
        r#"Ok(MethodCall { expr: Nested(TypeCast { target_type: "CMovingPhysicalAgentComponent", expr: MethodCall { expr: Nested(TypeCast { target_type: "CNewNPC", expr: ArrayAccess { expr: Identifier("results"), index: Identifier("i") } }), func: "GetMovingAgentComponent", args: [None] } }), func: "SetAnimatedMovement", args: [Some(Literal(Bool(false)))] })"#
    );
}