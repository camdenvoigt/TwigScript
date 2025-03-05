use crate::twig_interp::{interp_program, Types};
use crate::twig_parser::parse_program;
use pest::Parser;
use pest_derive::Parser;

pub mod twig_interp;
pub mod twig_parser;

#[derive(Parser)]
#[grammar = "grammars/grammar.pest"]
pub struct GrammarParser;

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
