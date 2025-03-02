use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammars/grammar.pest"]
struct GrammarParser;

#[derive(Debug)]
enum Expression {
    Integer(i32),
    Boolean(bool),
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
}

#[derive(Debug)]
enum Operator {
    Math(MathOperator),
    Bool(BooleanOperator),
}

#[derive(Debug)]
enum MathOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
enum BooleanOperator {
    Eqaul,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
}

fn parse_program(pairs: Pairs<Rule>) -> Expression {
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
            Rule::math_op => parse_program(primary.into_inner()),
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

fn interp_program(expr: Expression) -> i32 {
    match expr {
        Expression::Integer(value) => value,
        Expression::Boolean(value) => value as i32,
        Expression::MathOp { lhs, op, rhs } => {
            let left = interp_program(*lhs);
            let right = interp_program(*rhs);

            match op {
                MathOperator::Add => left + right,
                MathOperator::Subtract => left - right,
                MathOperator::Multiply => left * right,
                MathOperator::Divide => left / right,
            }
        }
        Expression::BooleanOp { lhs, op, rhs } => {
            let left = interp_program(*lhs);
            let right = interp_program(*rhs);

            match op {
                BooleanOperator::Eqaul => (left == right) as i32,
                BooleanOperator::GreaterThan => (left > right) as i32,
                BooleanOperator::GreaterThanEqual => (left >= right) as i32,
                BooleanOperator::LessThan => (left < right) as i32,
                BooleanOperator::LessThanEqual => (left <= right) as i32,
            }
        }
    }
}

fn main() {
    let program_input = "1 + 1 == 2";
    match GrammarParser::parse(Rule::program, program_input) {
        Ok(mut pairs) => {
            let program = parse_program(pairs.next().unwrap().into_inner());
            let result = interp_program(program);
            println!("Result: {}", result);
        }
        Err(e) => {
            println!("Program Parse Error: {}", e);
        }
    }
}
