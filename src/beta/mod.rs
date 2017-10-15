pub enum RuleExpression {
    Literal(Box<LiteralExpression>),
    Operator(Box<OperatorExpression>),
}

pub enum LiteralExpression {
    Num(i32),
    Str(String),
    Bool(bool),
}

pub struct OperatorExpression {
    operator_type: OperatorType,
    args: Vec<RuleExpression>,
}

pub enum OperatorType {
    And,
    Or,
    List,
}

mod eval;
