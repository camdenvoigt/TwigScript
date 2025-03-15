use crate::Rule;
use pest::iterators::{Pair, Pairs};
use pest::pratt_parser::PrattParser;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Integer(i32),
    Boolean(bool),
    String(String),
    Identifier(String),
    MathOp {
        lhs: Box<Expression>,
        op: MathOperator,
        rhs: Box<Expression>,
    },
    BooleanOp {
        lhs: Box<Expression>,
        op: BooleanOperator,
        rhs: Box<Expression>,
    },
    LetStmt {
        identifier: String,
        value: Box<Expression>,
    },
}

#[derive(Debug, PartialEq)]
enum Operator {
    Math(MathOperator),
    Bool(BooleanOperator),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MathOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BooleanOperator {
    Eqaul,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
}

fn get_operator(op: Pair<Rule>) -> Operator {
    match op.as_rule() {
        Rule::add => Operator::Math(MathOperator::Add),
        Rule::subtract => Operator::Math(MathOperator::Subtract),
        Rule::multiply => Operator::Math(MathOperator::Multiply),
        Rule::divide => Operator::Math(MathOperator::Divide),
        Rule::eq => Operator::Bool(BooleanOperator::Eqaul),
        Rule::gt => Operator::Bool(BooleanOperator::GreaterThan),
        Rule::ge => Operator::Bool(BooleanOperator::GreaterThanEqual),
        Rule::lt => Operator::Bool(BooleanOperator::LessThan),
        Rule::le => Operator::Bool(BooleanOperator::LessThanEqual),
        rule => unreachable!("Expected Operator rule found: {:?}", rule),
    }
}

fn get_operation(lhs: Expression, op: Operator, rhs: Expression) -> Expression {
    match op {
        Operator::Math(o) => Expression::MathOp {
            lhs: Box::new(lhs),
            op: o,
            rhs: Box::new(rhs),
        },
        Operator::Bool(o) => Expression::BooleanOp {
            lhs: Box::new(lhs),
            op: o,
            rhs: Box::new(rhs),
        },
    }
}

fn map_primary(primary: Pair<Rule>) -> Expression {
    match primary.as_rule() {
        Rule::integer => Expression::Integer(primary.as_str().parse().unwrap()),
        Rule::boolean => Expression::Boolean(primary.as_str() == "true"),
        Rule::string => Expression::String(String::from(primary.as_str())),
        Rule::identifier => Expression::Identifier(String::from(primary.as_str())),
        Rule::math_operation => parse_program(primary.into_inner()),
        Rule::boolean_operation => parse_program(primary.into_inner()),
        Rule::let_stmt => {
            let mut inner = primary.into_inner();
            let next = inner.next().unwrap().as_str();
            Expression::LetStmt {
                identifier: String::from(next),
                value: Box::new(parse_program(inner)),
            }
        }

        rule => unreachable!("Expected atomic rule found: {:?}", rule),
    }
}

pub fn parse_program(pairs: Pairs<Rule>) -> Expression {
    use pest::pratt_parser::{Assoc::*, Op};

    let parser = PrattParser::new()
        .op(Op::infix(Rule::add, Left) | Op::infix(Rule::subtract, Left))
        .op(Op::infix(Rule::multiply, Left) | Op::infix(Rule::divide, Left))
        .op(Op::infix(Rule::eq, Left)
            | Op::infix(Rule::gt, Left)
            | Op::infix(Rule::ge, Left)
            | Op::infix(Rule::lt, Left)
            | Op::infix(Rule::le, Left));

    parser
        .map_primary(map_primary)
        .map_infix(|lhs, op, rhs| get_operation(lhs, get_operator(op), rhs))
        .parse(pairs)
}

#[cfg(test)]
mod twig_parser_tests {
    use super::*;
    use pest::{ParseResult, ParserState};

    #[test]
    fn test_get_operator_add() {
        let input = "+";
        let pair = pest::state(input, |state| state.rule(Rule::add, |s| Ok(s)))
            .unwrap()
            .next()
            .unwrap();
        let result = get_operator(pair);

        assert_eq!(result, Operator::Math(MathOperator::Add));
    }

    #[test]
    fn test_get_operator_sub() {
        let input = "-";
        let pair = pest::state(input, |state| state.rule(Rule::subtract, |s| Ok(s)))
            .unwrap()
            .next()
            .unwrap();
        let result = get_operator(pair);

        assert_eq!(result, Operator::Math(MathOperator::Subtract));
    }

    #[test]
    fn test_get_operator_mult() {
        let input = "*";
        let pair = pest::state(input, |state| state.rule(Rule::multiply, |s| Ok(s)))
            .unwrap()
            .next()
            .unwrap();
        let result = get_operator(pair);

        assert_eq!(result, Operator::Math(MathOperator::Multiply));
    }

    #[test]
    fn test_get_operator_div() {
        let input = "/";
        let pair = pest::state(input, |state| state.rule(Rule::divide, |s| Ok(s)))
            .unwrap()
            .next()
            .unwrap();
        let result = get_operator(pair);

        assert_eq!(result, Operator::Math(MathOperator::Divide));
    }

    #[test]
    fn test_get_operator_eq() {
        let input = "==";
        let pair = pest::state(input, |state| state.rule(Rule::eq, |s| Ok(s)))
            .unwrap()
            .next()
            .unwrap();
        let result = get_operator(pair);

        assert_eq!(result, Operator::Bool(BooleanOperator::Eqaul));
    }

    #[test]
    fn test_get_operator_gt() {
        let input = ">";
        let pair = pest::state(input, |state| state.rule(Rule::gt, |s| Ok(s)))
            .unwrap()
            .next()
            .unwrap();
        let result = get_operator(pair);

        assert_eq!(result, Operator::Bool(BooleanOperator::GreaterThan));
    }

    #[test]
    fn test_get_operator_ge() {
        let input = ">=";
        let pair = pest::state(input, |state| state.rule(Rule::ge, |s| Ok(s)))
            .unwrap()
            .next()
            .unwrap();
        let result = get_operator(pair);

        assert_eq!(result, Operator::Bool(BooleanOperator::GreaterThanEqual));
    }

    #[test]
    fn test_get_operator_lt() {
        let input = "<";
        let pair = pest::state(input, |state| state.rule(Rule::lt, |s| Ok(s)))
            .unwrap()
            .next()
            .unwrap();
        let result = get_operator(pair);

        assert_eq!(result, Operator::Bool(BooleanOperator::LessThan));
    }

    #[test]
    fn test_get_operator_le() {
        let input = "<=";
        let pair = pest::state(input, |state| state.rule(Rule::le, |s| Ok(s)))
            .unwrap()
            .next()
            .unwrap();
        let result = get_operator(pair);

        assert_eq!(result, Operator::Bool(BooleanOperator::LessThanEqual));
    }

    #[test]
    fn test_get_operation_math() {
        let op = MathOperator::Subtract;
        let result = get_operation(
            Expression::Integer(1),
            Operator::Math(op),
            Expression::Integer(1),
        );

        let expected = Expression::MathOp {
            lhs: Box::new(Expression::Integer(1)),
            op,
            rhs: Box::new(Expression::Integer(1)),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_operation_bool() {
        let op = BooleanOperator::LessThan;
        let result = get_operation(
            Expression::Integer(1),
            Operator::Bool(op),
            Expression::Integer(1),
        );

        let expected = Expression::BooleanOp {
            lhs: Box::new(Expression::Integer(1)),
            op,
            rhs: Box::new(Expression::Integer(1)),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_map_primary_int() {
        let input = "8";
        let rule = Rule::integer;
        fn parser_rules(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
            state.match_string("8")
        }
        let pair = pest::state(input, |state| state.rule(rule, parser_rules))
            .unwrap()
            .next()
            .unwrap();

        let result = map_primary(pair);
        let expected = Expression::Integer(8);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_map_primary_bool() {
        let input = "true";
        let rule = Rule::boolean;
        fn parser_rules(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
            state.match_string("true")
        }
        let pair = pest::state(input, |state| state.rule(rule, parser_rules))
            .unwrap()
            .next()
            .unwrap();

        let result = map_primary(pair);
        let expected = Expression::Boolean(true);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_map_primary_string() {
        let input = "test";
        let rule = Rule::string;
        fn parser_rules(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
            state.match_string("test")
        }
        let pair = pest::state(input, |state| state.rule(rule, parser_rules))
            .unwrap()
            .next()
            .unwrap();

        let result = map_primary(pair);
        let expected = Expression::String(String::from("test"));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_map_primary_id() {
        let input = "test";
        let rule = Rule::identifier;
        fn parser_rules(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
            state.match_string("test")
        }
        let pair = pest::state(input, |state| state.rule(rule, parser_rules))
            .unwrap()
            .next()
            .unwrap();

        let result = map_primary(pair);
        let expected = Expression::Identifier(String::from("test"));

        assert_eq!(result, expected);
    }
}
