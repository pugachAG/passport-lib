use nom::{digit, is_alphanumeric, IResult};
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

named!(int <i32>,
  map_res!(
    map_res!(
      digit,
      str::from_utf8
    ),
    FromStr::from_str
  )
);

named!(string <&str>,
  map_res!(
    delimited!(
      tag!("\""),
      escaped!(
        is_not!("\\\""),
        '\\',
        one_of!("\\\"")
      ),
      tag!("\"")
    ),
    str::from_utf8
  )
);

named!(boolean <bool>,
  map_res!(
    map_res!(
      alt!(
        tag!("true") | tag!("false")
      ),
      str::from_utf8
    ),
    FromStr::from_str
  )
);

named!(literal <LiteralExpression>,
  alt_complete!(
    int  => { |i| LiteralExpression::Num(i) } |
    string => { |s| LiteralExpression::Str(String::from(s)) } |
    boolean => { |b| LiteralExpression::Bool(b) }
  )
);

fn parse_func_name(name: &str) -> Result<OperatorType, String> {
    let op = match name {
        "or" => OperatorType::Or,
        "and" => OperatorType::And,
        _ => return Result::Err(String::from(format!("Unknown operator {}", name)))
    };
    Result::Ok(op)
}

named!(args <Vec<RuleExpression>>,
  separated_list!(
    tag!(","),
    rule_expr
  )
);

named!(func <OperatorExpression>,
  do_parse!(
    op_type: map_res!(
      map_res!(
        take_while!( |x| is_alphanumeric(x) || x == b'_' ),
        str::from_utf8
      ),
      parse_func_name
    ) >>
    tag!("(") >>
    f_args: args >>
    tag!(")") >>
    (OperatorExpression{ operator_type: op_type, args: f_args })
  )
);

named!(list <OperatorExpression>,
  do_parse!(
    tag!("[") >>
    l_args: args >>
    tag!("]") >>
    (OperatorExpression{ operator_type: OperatorType::List, args: l_args })
  )
);

named!(operator <OperatorExpression>,
  alt_complete!(
    func | list
  )
);

named!(rule_expr <RuleExpression>,
  ws!(
    alt_complete!(
        operator => { |e| RuleExpression::Operator(Box::new(e)) } |
        literal => { |e| RuleExpression::Literal(Box::new(e))  }
    )
  )
);

fn parse_expr(text: &str) -> RuleExpression {
    match rule_expr(text.as_bytes()) {
        IResult::Done(_, res) => res,
        _ => panic!("Error parsing")
    }
}

mod eval;

#[cfg(test)]
mod tests {
    use super::*;
    use super::eval::*;
    use super::eval::PrimitiveValue::*;
    use super::eval::ExpressionValue::*;

    #[test]
    fn num_literal() {
        let expr = parse_expr("228");
        if let Single(Num(val)) = eval_expr(&expr) {
            assert_eq!(val, 228);
        } else {
            panic!("Expected single number");
        }
    }

    #[test]
    fn str_literal() {
        let expr = parse_expr("\"str 123!\"");
        if let Single(Str(s)) = eval_expr(&expr) {
            assert_eq!(s, "str 123!");
        } else {
            panic!("Expected single string");
        }
    }

    #[test]
    fn bool_literal() {
        let expr = parse_expr("false");
        if let Single(Bool(b)) = eval_expr(&expr) {
            assert_eq!(b, false);
        } else {
            panic!("Expected single bool");
        }
    }

    #[test]
    fn list_operator() {
        let expr = parse_expr("[1, 2]");
        if let Repeated(lst) = eval_expr(&expr) {
            let v: Vec<_> = lst.iter()
                .map(|x| if let &Num(ref v) = x { *v } else { 0 })
                .collect();
            assert_eq!(v, vec![1, 2]);
        } else {
            panic!("Expected repeated");
        }
    }

    #[test]
    fn nested_operators() {
        let expr = parse_expr("and(true, or(true, false), and(true, true))");
        if let Single(Bool(b)) = eval_expr(&expr) {
            assert_eq!(b, true);
        } else {
            panic!("Expected single bool");
        }
    }
}
