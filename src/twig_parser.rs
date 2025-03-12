use crate::Rule;
use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;

#[derive(Debug)]
pub enum Expression {
    Integer(i32),
    Boolean(bool),
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
        identifier: Box<String>,
        value: Box<Expression>,
    },
}

#[derive(Debug)]
enum Operator {
    Math(MathOperator),
    Bool(BooleanOperator),
}

#[derive(Debug)]
pub enum MathOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub enum BooleanOperator {
    Eqaul,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
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

    let result = parser
        .map_primary(|primary| match primary.as_rule() {
            Rule::integer => Expression::Integer(primary.as_str().parse().unwrap()),
            Rule::boolean => Expression::Boolean(primary.as_str() == "true"),
            Rule::math_operation => parse_program(primary.into_inner()),
            Rule::boolean_operation => parse_program(primary.into_inner()),
            Rule::identifier => Expression::Identifier(String::from(primary.as_str())),
            Rule::let_stmt => {
                let mut inner = primary.into_inner();
                let next = inner.next().unwrap().as_str();
                Expression::LetStmt {
                    identifier: Box::new(String::from(next)),
                    value: Box::new(parse_program(inner)),
                }
            }

            rule => unreachable!("Expected atomic rule found: {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let operator = match op.as_rule() {
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
            };
            match operator {
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
        })
        .parse(pairs);

    result
}
