use core::fmt;
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

#[derive(Debug)]
enum Types {
    Integer(i32),
    Boolean(bool),
}

#[derive(Debug)]
enum InterpErrors {
    MismatchedTypeError,
    InvalidTypeError,
}

impl fmt::Display for InterpErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            InterpErrors::MismatchedTypeError => write!(f, "Mismatched types"),
            InterpErrors::InvalidTypeError => write!(f, "InvalidTypeError"),
        }
    }
}

impl std::error::Error for InterpErrors {}

fn parse_program(pairs: Pairs<Rule>) -> Expression {
    use pest::pratt_parser::{Assoc::*, Op};

    let parser = PrattParser::new()
        .op(Op::infix(Rule::eq, Left)
            | Op::infix(Rule::gt, Left)
            | Op::infix(Rule::ge, Left)
            | Op::infix(Rule::lt, Left)
            | Op::infix(Rule::le, Left))
        .op(Op::infix(Rule::add, Left) | Op::infix(Rule::subtract, Left))
        .op(Op::infix(Rule::multiply, Left) | Op::infix(Rule::divide, Left));

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

fn interp_program(expr: Expression) -> std::result::Result<Types, InterpErrors> {
    match expr {
        Expression::Integer(value) => Ok(Types::Integer(value)),
        Expression::Boolean(value) => Ok(Types::Boolean(value)),
        Expression::MathOp { lhs, op, rhs } => {
            let (Ok(Types::Integer(left)), Ok(Types::Integer(right))) =
                (interp_program(*lhs), interp_program(*rhs))
            else {
                return Err(InterpErrors::InvalidTypeError);
            };

            let result = match op {
                MathOperator::Add => left + right,
                MathOperator::Subtract => left - right,
                MathOperator::Multiply => left * right,
                MathOperator::Divide => left / right,
            };

            Ok(Types::Integer(result))
        }
        Expression::BooleanOp { lhs, op, rhs } => {
            let (Ok(l), Ok(r)) = (interp_program(*lhs), interp_program(*rhs)) else {
                return Err(InterpErrors::InvalidTypeError);
            };

            let (left, right) = match (l, r) {
                (Types::Integer(i), Types::Integer(j)) => (i, j),
                (Types::Boolean(i), Types::Boolean(j)) => (i as i32, j as i32),
                _ => return Err(InterpErrors::MismatchedTypeError),
            };

            let result = match op {
                BooleanOperator::Eqaul => left == right,
                BooleanOperator::GreaterThan => left > right,
                BooleanOperator::GreaterThanEqual => left >= right,
                BooleanOperator::LessThan => left < right,
                BooleanOperator::LessThanEqual => left <= right,
            };

            Ok(Types::Boolean(result))
        }
    }
}

fn main() {
    let program_input = "false > true";
    match GrammarParser::parse(Rule::program, program_input) {
        Ok(mut pairs) => {
            let program = parse_program(pairs.next().unwrap().into_inner());
            let result = interp_program(program);
            match result {
                Ok(Types::Integer(i)) => println!("Integer Result: {}", i),
                Ok(Types::Boolean(b)) => println!("Boolean Result: {}", b),
                Err(e) => println!("{}", e),
            }
        }
        Err(e) => {
            println!("Program Parse Error: {}", e);
        }
    }
}
