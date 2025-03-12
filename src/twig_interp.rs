use core::fmt;
use std::collections::BTreeMap;

use crate::twig_parser::{BooleanOperator, Expression, MathOperator};

#[derive(Debug, Clone, Copy)]
pub enum Types {
    Integer(i32),
    Boolean(bool),
    Unit,
}

#[derive(Debug)]
pub enum InterpErrors {
    MismatchedTypeError,
    InvalidTypeError,
    VariableDoesNotExist,
}

pub type Env = BTreeMap<Box<String>, Box<Types>>;

impl fmt::Display for InterpErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            InterpErrors::MismatchedTypeError => write!(f, "Mismatched types"),
            InterpErrors::InvalidTypeError => write!(f, "InvalidTypeError"),
            InterpErrors::VariableDoesNotExist => write!(f, "Variable does not exist"),
        }
    }
}

impl std::error::Error for InterpErrors {}

pub fn interp_program(expr: Expression, env: &mut Env) -> std::result::Result<Types, InterpErrors> {
    match expr {
        Expression::Integer(value) => Ok(Types::Integer(value)),
        Expression::Boolean(value) => Ok(Types::Boolean(value)),
        Expression::MathOp { lhs, op, rhs } => {
            let (Ok(Types::Integer(left)), Ok(Types::Integer(right))) =
                (interp_program(*lhs, env), interp_program(*rhs, env))
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
            let (Ok(l), Ok(r)) = (interp_program(*lhs, env), interp_program(*rhs, env)) else {
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
        Expression::LetStmt { identifier, value } => match interp_program(*value, env) {
            Ok(result) => {
                env.insert(identifier, Box::new(result));
                Ok(Types::Unit)
            }
            Err(e) => Err(e),
        },
        Expression::Identifier(var) => match env.get(&var) {
            Some(value) => Ok(**value),
            None => Err(InterpErrors::VariableDoesNotExist),
        },
    }
}
