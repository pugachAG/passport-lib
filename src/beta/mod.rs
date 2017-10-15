mod beta {
    enum RuleExpression {
        Literal(Box<LiteralExpression>),
        Operator(Box<OperatorExpression>),
    }

    enum LiteralExpression {
        Numeric(i32),
        String(String),
        Boolean(bool),
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