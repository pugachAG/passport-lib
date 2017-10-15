//mod eval;

mod beta {
    enum RuleExpression {
        Literal(Box<LiteralExpression>),
        Operator(Box<OperatorExpression>),
    }

    enum LiteralExpression {
        Num(i32),
        Str(String),
        Bool(bool),
    }

    struct OperatorExpression {
        operator_type: OperatorType,
        args: Vec<RuleExpression>,
    }

    enum OperatorType {
        And,
        Or,
        List,
    }
}