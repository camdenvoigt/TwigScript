use pest::pratt_parser::PrattParser;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammars/grammar.pest"]
struct GrammarParser;

#[derive(Debug)]
enum Expression {
    Integer(i32),
    BinOp {
        lhs: Box<Expression>,
        op: Operator,
        rhs: Box<Expression>,
    },
}

#[derive(Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn parse_program() -> Expression {
    let mut pairs = GrammarParser::parse(Rule::program, "5 * 5 + 10").unwrap();
    use pest::pratt_parser::{Assoc::*, Op};

    let parser = PrattParser::new()
        .op(Op::infix(Rule::add, Left) | Op::infix(Rule::subtract, Left))
        .op(Op::infix(Rule::multiply, Left) | Op::infix(Rule::divide, Left));

    let result = parser
        .map_primary(|primary| match primary.as_rule() {
            Rule::integer => Expression::Integer(primary.as_str().parse().unwrap()),
            rule => unreachable!("Expected atomic rule found: {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let operator = match op.as_rule() {
                Rule::add => Operator::Add,
                Rule::subtract => Operator::Subtract,
                Rule::multiply => Operator::Multiply,
                Rule::divide => Operator::Divide,
                rule => unreachable!("Expected Operator rule found: {:?}", rule),
            };
            Expression::BinOp {
                lhs: Box::new(lhs),
                op: operator,
                rhs: Box::new(rhs),
            }
        })
        .parse(pairs.next().unwrap().into_inner());

    result
}

fn interp_program(expr: Expression) -> i32 {
    match expr {
        Expression::Integer(value) => value,
        Expression::BinOp { lhs, op, rhs } => {
            let left = interp_program(*lhs);
            let right = interp_program(*rhs);
            println!("left: {}, right: {}", left, right);
            match op {
                Operator::Add => left + right,
                Operator::Subtract => left - right,
                Operator::Multiply => left * right,
                Operator::Divide => left / right,
            }
        }
    }
}

fn main() {
    let program = parse_program();
    let result = interp_program(program);
    println!("Result: {}", result);
}
