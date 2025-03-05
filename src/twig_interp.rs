use core::fmt;

use crate::twig_parser::{BooleanOperator, Expression, MathOperator};

#[derive(Debug)]
pub enum Types {
    Integer(i32),
    Boolean(bool),
}

#[derive(Debug)]
pub enum InterpErrors {
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

pub fn interp_program(expr: Expression) -> std::result::Result<Types, InterpErrors> {
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
