use super::*;

#[derive(Clone)]
pub enum PrimitiveValue {
    Num(i32),
    Str(String),
    Bool(bool),
}

pub enum ExpressionValue {
    Single(PrimitiveValue),
    Repeated(Vec<PrimitiveValue>),
}

impl ExpressionValue {
    fn to_bool(&self) -> bool {
        match self {
            &ExpressionValue::Single(PrimitiveValue::Bool(ref b)) => b.clone(),
            _ => panic!("Single Bool expected"),
        }
    }

    fn to_single(&self) -> PrimitiveValue {
        match self {
            &ExpressionValue::Single(ref prim) => prim.clone(),
            _ => panic!("Single expected"),
        }
    }
}

pub fn eval_expr(rule_expr: &RuleExpression) -> ExpressionValue {
    match rule_expr {
        &RuleExpression::Literal(ref literal) => ExpressionValue::Single(eval_literal(literal)),
        &RuleExpression::Operator(ref operator) => eval_operator(operator),
    }
}

fn eval_literal(literal_expr: &LiteralExpression) -> PrimitiveValue {
    match literal_expr {
        &LiteralExpression::Num(ref num) => PrimitiveValue::Num(*num),
        &LiteralExpression::Str(ref s) => PrimitiveValue::Str(s.clone()),
        &LiteralExpression::Bool(ref b) => PrimitiveValue::Bool(*b),
    }
}

fn eval_operator(operator_expr: &OperatorExpression) -> ExpressionValue {
    let arg_results = operator_expr.args.iter()
        .map(|arg| eval_expr(arg));

    match operator_expr.operator_type {
        OperatorType::And => ExpressionValue::Single(PrimitiveValue::Bool(
            arg_results.map(|ev| ev.to_bool()).all(|b| b))),

        OperatorType::Or => ExpressionValue::Single(PrimitiveValue::Bool(
            arg_results.map(|ev| ev.to_bool()).any(|b| b))),

        OperatorType::List => ExpressionValue::Repeated(
            arg_results.map(|ev| ev.to_single()).collect::<Vec<_>>())
    }
}

