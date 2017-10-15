use nom::{digit, alphanumeric, IResult};
use std::str::{self, FromStr};

#[derive(Debug)]
pub enum RuleExpression {
    Literal(Box<LiteralExpression>),
    Operator(Box<OperatorExpression>),
}

#[derive(Debug)]
pub enum LiteralExpression {
    Num(i32),
    Str(String),
    Bool(bool),
}

#[derive(Debug)]
pub struct OperatorExpression {
    operator_type: OperatorType,
    args: Vec<RuleExpression>,
}

#[derive(Debug)]
pub enum OperatorType {
    And,
    Or,
    List,
}

named!(int <i32>, map_res!(
  map_res!(
    recognize!(
        many1!(digit)
    ),
    str::from_utf8
  ),
  FromStr::from_str
));

named!(string<&str>,
  delimited!(
    tag!("\""),
    map_res!(escaped!(call!(alphanumeric), '\\', is_a!("\"n\\")), str::from_utf8),
    tag!("\"")
  )
);

named!(boolean <bool>, map_res!(
  map_res!(
    recognize!(
        alt!(
            tag!("true") |
            tag!("false")
        )
    ),
    str::from_utf8
  ),
  FromStr::from_str
));

named!(arg <LiteralExpression>,
  ws!(
    alt!(
      int     => { |i|   LiteralExpression::Num(i) } |
      string  => { |s|   LiteralExpression::Str(String::from(s)) } |
      boolean => { |b|   LiteralExpression::Bool(b)      }
    )
  )
);

named!(and_op <OperatorExpression>,
  ws!(
    do_parse!(
      tag!("and") >>
      tag!("(") >>
      args: separated_list!(tag!(","), rule_expr) >>
      tag!(")") >>
      (OperatorExpression{ operator_type: OperatorType::And, args: args })
    )
  )
);

named!(or_op <OperatorExpression>,
  ws!(
    do_parse!(
      tag!("or") >>
      tag!("(") >>
      args: separated_list!(tag!(","), rule_expr) >>
      tag!(")") >>
      (OperatorExpression{ operator_type: OperatorType::Or, args: args })
    )
  )
);

named!(list_op <OperatorExpression>,
  ws!(
    do_parse!(
      tag!("[") >>
      args: separated_list!(tag!(","), rule_expr) >>
      tag!("]") >>
      (OperatorExpression{ operator_type: OperatorType::List, args: args })
    )
  )
);

named!(oper_expr <OperatorExpression>,
  ws!(
    alt!(
      list_op |
      or_op |
      and_op
    )
  )
);

named!(rule_expr <RuleExpression>,
  ws!(
    alt!(
        oper_expr => { |e| RuleExpression::Operator(Box::new(e)) } |
        arg       => { |e| RuleExpression::Literal(Box::new(e))  }
    )
  )
);

fn parse_expr(text: &str) -> RuleExpression {
    match rule_expr(text.as_bytes()) {
        IResult::Done(_, res) => res,
        _ => RuleExpression::Literal(Box::new(LiteralExpression::Str(String::from("error"))))
    }
}

mod eval;
