use beta::*;

pub enum PrimitiveValue {
    Num(i32),
    Str(String),
    Bool(bool),
}

pub enum ExpressionValue {
    Single(PrimitiveValue),
    Repeated(Vec<PrimitiveValue>),
}

pub fn eval_expr(rule_expr: &RuleExpression) -> ExpressionValue {
    match rule_expr {
        &RuleExpression::Literal(ref literal) => ExpressionValue::Single(eval_literal(literal)),
        &RuleExpression::Operator(ref operator) => eval_operator(operator),
    }
}

fn eval_literal(literal_expr: &LiteralExpression) -> PrimitiveValue {
    match literal_expr {
        &LiteralExpression::Num(ref num) => PrimitiveValue::Num(num.clone()),
        &LiteralExpression::Str(ref s) => PrimitiveValue::Str(s.clone()),
        &LiteralExpression::Bool(ref b) => PrimitiveValue::Bool(b.clone()),
    }
}

fn eval_operator(operator_expr: &OperatorExpression) -> ExpressionValue {
    ExpressionValue::Single(PrimitiveValue::Num(0))
}
