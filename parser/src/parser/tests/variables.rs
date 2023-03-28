use super::{build_parse_test, Identifier, IntegrityConstraint, Source, SourceSection};
use crate::ast::{
    ConstraintType, Expression::*, IntegrityStmt::*, TraceBindingAccess, TraceBindingAccessSize,
    Variable, VariableType, VectorAccess,
};

// VARIABLES
// ================================================================================================
#[test]
fn variables_with_and_operators() {
    let source = "
    integrity_constraints:
        let flag = n1 & !n2
        enf clk' = clk + 1 when flag";
    let expected = Source(vec![SourceSection::IntegrityConstraints(vec![
        Variable(Variable::new(
            Identifier("flag".to_string()),
            VariableType::Scalar(Mul(
                Box::new(Elem(Identifier("n1".to_string()))),
                Box::new(Sub(
                    Box::new(Const(1)),
                    Box::new(Elem(Identifier("n2".to_string()))),
                )),
            )),
        )),
        Constraint(
            ConstraintType::Inline(IntegrityConstraint::new(
                TraceBindingAccess(TraceBindingAccess::new(
                    Identifier("clk".to_string()),
                    0,
                    TraceBindingAccessSize::Full,
                    1,
                )),
                Add(
                    Box::new(Elem(Identifier("clk".to_string()))),
                    Box::new(Const(1)),
                ),
            )),
            Some(Elem(Identifier("flag".to_string()))),
        ),
    ])]);
    build_parse_test!(source).expect_ast(expected);
}

#[test]
fn variables_with_or_operators() {
    let source = "
    integrity_constraints:
        let flag = s[0] | !s[1]'
        enf clk' = clk + 1 when flag";
    let expected = Source(vec![SourceSection::IntegrityConstraints(vec![
        Variable(Variable::new(
            Identifier("flag".to_string()),
            VariableType::Scalar(Sub(
                Box::new(Add(
                    Box::new(VectorAccess(VectorAccess::new(
                        Identifier("s".to_string()),
                        0,
                    ))),
                    Box::new(Sub(
                        Box::new(Const(1)),
                        Box::new(TraceBindingAccess(TraceBindingAccess::new(
                            Identifier("s".to_string()),
                            1,
                            TraceBindingAccessSize::Single,
                            1,
                        ))),
                    )),
                )),
                Box::new(Mul(
                    Box::new(VectorAccess(VectorAccess::new(
                        Identifier("s".to_string()),
                        0,
                    ))),
                    Box::new(Sub(
                        Box::new(Const(1)),
                        Box::new(TraceBindingAccess(TraceBindingAccess::new(
                            Identifier("s".to_string()),
                            1,
                            TraceBindingAccessSize::Single,
                            1,
                        ))),
                    )),
                )),
            )),
        )),
        Constraint(
            ConstraintType::Inline(IntegrityConstraint::new(
                TraceBindingAccess(TraceBindingAccess::new(
                    Identifier("clk".to_string()),
                    0,
                    TraceBindingAccessSize::Full,
                    1,
                )),
                Add(
                    Box::new(Elem(Identifier("clk".to_string()))),
                    Box::new(Const(1)),
                ),
            )),
            Some(Elem(Identifier("flag".to_string()))),
        ),
    ])]);
    build_parse_test!(source).expect_ast(expected);
}

// VARIABLES INVALID USAGE
// ================================================================================================

#[test]
fn err_vector_defined_outside_boundary_or_integrity_constraints() {
    let source = "
        const A = 1
        let a = 0";
    build_parse_test!(source).expect_unrecognized_token();
}

#[test]
fn err_vector_variable_with_trailing_comma() {
    let source = "
    integrity_constraints:
        let a = [1, ]";
    build_parse_test!(source).expect_unrecognized_token();
}

#[test]
fn err_matrix_variable_with_trailing_comma() {
    let source = "
    integrity_constraints:
        let a = [[1, 2], ]";
    build_parse_test!(source).expect_unrecognized_token();
}

#[test]
fn err_matrix_variable_mixed_element_types() {
    let source = "integrity_constraints:
    let a = [[1, 2], 1]";
    build_parse_test!(source).expect_unrecognized_token();
}

#[test]
fn err_invalid_matrix_element() {
    let source = "integrity_constraints:
    let a = [[1, 2], [3, [4, 5]]]";
    build_parse_test!(source).expect_unrecognized_token();
}

#[test]
fn err_matrix_variable_from_vector_and_reference() {
    let source = "integrity_constraints:
    let a = [[1, 2], [3, 4]]
    let b = [5, 6]
    let c = [b, [7, 8]]
    let d = [[7, 8], a[0]]";
    build_parse_test!(source).expect_unrecognized_token();
}