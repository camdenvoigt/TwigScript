use pest::iterators::Pair;
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
        op: Op,
        rhs: Box<Expression>,
    },
}

#[derive(Debug)]
enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn parse_op_pair(pair: Pair<Rule>) -> Op {
    match pair.as_rule() {
        Rule::add => Op::Add,
        Rule::subtract => Op::Subtract,
        Rule::multiply => Op::Multiply,
        Rule::divide => Op::Divide,
        _ => panic!("Not Op type pair"),
    }
}

fn parse_expression_pair(pair: Pair<'_, Rule>) -> Expression {
    println!("Current pair: {}", pair.as_str());
    match pair.as_rule() {
        Rule::integer => Expression::Integer(pair.as_str().parse().unwrap()),
        Rule::math_op => {
            let mut inner_pairs = pair.into_inner();
            Expression::BinOp {
                lhs: Box::new(parse_expression_pair(inner_pairs.next().unwrap())),
                op: parse_op_pair(inner_pairs.next().unwrap()),
                rhs: Box::new(parse_expression_pair(inner_pairs.next().unwrap())),
            }
        }
        Rule::EOI => Expression::Integer(0),
        _ => unreachable!(),
    }
}

fn parse_program() -> Vec<Expression> {
    let pairs = GrammarParser::parse(Rule::program, "5 * 5 + 10").unwrap();
    let mut expression_trees = Vec::new();
    println!("{:?}", pairs);
    for pair in pairs {
        expression_trees.push(parse_expression_pair(pair));
    }
    println!("{:?}", expression_trees);
    expression_trees
}

fn interp_program(expr: Expression) -> i32 {
    match expr {
        Expression::Integer(value) => value,
        Expression::BinOp { lhs, op, rhs } => {
            let left = interp_program(*lhs);
            let right = interp_program(*rhs);
            println!("left: {}, right: {}", left, right);
            match op {
                Op::Add => left + right,
                Op::Subtract => left - right,
                Op::Multiply => left * right,
                Op::Divide => left / right,
            }
        }
    }
}

fn main() {
    let mut program = parse_program();
    let result = interp_program(program.remove(0));
    println!("Result: {}", result);
}
