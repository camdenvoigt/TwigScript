use crate::twig_interp::{Env, Types, interp_program};
use crate::twig_parser::parse_program;
use pest::Parser;
use pest_derive::Parser;
use std::io::{self, BufRead, Write};

pub mod twig_interp;
pub mod twig_parser;

#[derive(Parser)]
#[grammar = "grammars/grammar.pest"]
pub struct GrammarParser;

fn run_input(program_input: &str, env: &mut Env) {
    match GrammarParser::parse(Rule::program, program_input) {
        Ok(mut pairs) => {
            let program = parse_program(pairs.next().unwrap().into_inner());
            let result = interp_program(program, env);
            match result {
                Ok(Types::Integer(i)) => println!("Integer Result: {}", i),
                Ok(Types::Boolean(b)) => println!("Boolean Result: {}", b),
                Ok(Types::String(s)) => println!("String Result: {}", s),
                Ok(Types::Unit) => println!("Unit Result"),
                Err(e) => println!("{}", e),
            }
        }
        Err(e) => {
            println!("Program Parse Error: {}", e);
        }
    }
}

fn write_indicator() {
    io::stdout().write_all(b"> ").unwrap();
    io::stdout().flush().unwrap();
}

fn main() {
    let stdin = io::stdin();
    let mut env = Env::new();
    write_indicator();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l == "exit" {
            std::process::exit(1);
        }
        run_input(l.as_str(), &mut env);
        write_indicator();
    }
}
