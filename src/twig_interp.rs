use core::fmt;
use std::collections::BTreeMap;

use crate::twig_parser::{BooleanOperator, Expression, MathOperator};

#[derive(Debug, Clone, PartialEq)]
pub enum Types {
    Integer(i32),
    Boolean(bool),
    String(String),
    Unit,
}

#[derive(Debug, PartialEq)]
pub enum InterpErrors {
    MismatchedTypeError,
    InvalidTypeError,
    VariableDoesNotExist,
}

pub type Env = BTreeMap<String, Box<Types>>;

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
        Expression::String(value) => Ok(Types::String(value)),
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
                (Types::String(_), Types::String(_)) => return Err(InterpErrors::InvalidTypeError),
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
            // Need to do bit of a strange dereference here to get Types out of Box. Clone to get a
            // copy of the value in env.
            Some(value) => Ok(*(*value).clone()),
            None => Err(InterpErrors::VariableDoesNotExist),
        },
    }
}

#[cfg(test)]
mod twig_interp_tests {
    use super::*;

    #[test]
    fn test_interp_program_int() {
        let mut env = Env::new();
        let e = Expression::Integer(1);

        let result = interp_program(e, &mut env).unwrap();
        let expected = Types::Integer(1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_interp_program_bool() {
        let mut env = Env::new();
        let e = Expression::Boolean(true);

        let result = interp_program(e, &mut env).unwrap();
        let expected = Types::Boolean(true);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_interp_program_string() {
        let mut env = Env::new();
        let e = Expression::String(String::from("string"));

        let result = interp_program(e, &mut env).unwrap();
        let expected = Types::String(String::from("string"));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_interp_program_int_id() {
        let mut env = Env::new();
        let id = "a";
        env.insert(String::from(id), Box::new(Types::Integer(1)));
        let e = Expression::Identifier(String::from(id));

        let result = interp_program(e, &mut env).unwrap();
        let expected = Types::Integer(1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_interp_program_bool_id() {
        let mut env = Env::new();
        let id = "a";
        env.insert(String::from(id), Box::new(Types::Boolean(true)));
        let e = Expression::Identifier(String::from(id));

        let result = interp_program(e, &mut env).unwrap();
        let expected = Types::Boolean(true);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_interp_program_string_id() {
        let mut env = Env::new();
        let id = "a";
        let value = "value";
        env.insert(
            String::from(id),
            Box::new(Types::String(String::from(value))),
        );
        let e = Expression::Identifier(String::from(id));

        let result = interp_program(e, &mut env).unwrap();
        let expected = Types::String(String::from(value));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_interp_program_bad_id() {
        let mut env = Env::new();
        let id = "a";
        let value = "value";
        env.insert(
            String::from("b"),
            Box::new(Types::String(String::from(value))),
        );
        let e = Expression::Identifier(String::from(id));

        let result = interp_program(e, &mut env);

        assert_eq!(result.unwrap_err(), InterpErrors::VariableDoesNotExist);
    }

    #[test]
    fn test_interp_program_let_stmt() {
        let mut env = Env::new();
        let id = "a";
        let e = Expression::LetStmt {
            identifier: String::from(id),
            value: Box::new(Expression::Integer(1)),
        };

        let result = interp_program(e, &mut env).unwrap();
        let env_result = env.get(&String::from(id)).unwrap();

        assert_eq!(result, Types::Unit);
        assert_eq!(**env_result, Types::Integer(1));
    }

    #[test]
    fn test_interp_program_math_op_add() {
        let mut env = Env::new();
        let e = Expression::MathOp {
            lhs: Box::new(Expression::Integer(1)),
            op: MathOperator::Add,
            rhs: Box::new(Expression::Integer(1)),
        };

        let result = interp_program(e, &mut env).unwrap();

        assert_eq!(result, Types::Integer(2));
    }

    #[test]
    fn test_interp_program_math_op_sub() {
        let mut env = Env::new();
        let e = Expression::MathOp {
            lhs: Box::new(Expression::Integer(1)),
            op: MathOperator::Subtract,
            rhs: Box::new(Expression::Integer(1)),
        };

        let result = interp_program(e, &mut env).unwrap();

        assert_eq!(result, Types::Integer(0));
    }

    #[test]
    fn test_interp_program_math_op_mult() {
        let mut env = Env::new();
        let e = Expression::MathOp {
            lhs: Box::new(Expression::Integer(2)),
            op: MathOperator::Multiply,
            rhs: Box::new(Expression::Integer(3)),
        };

        let result = interp_program(e, &mut env).unwrap();

        assert_eq!(result, Types::Integer(6));
    }

    #[test]
    fn test_interp_program_math_op_div() {
        let mut env = Env::new();
        let e = Expression::MathOp {
            lhs: Box::new(Expression::Integer(100)),
            op: MathOperator::Divide,
            rhs: Box::new(Expression::Integer(10)),
        };

        let result = interp_program(e, &mut env).unwrap();

        assert_eq!(result, Types::Integer(10));
    }

    #[test]
    fn test_interp_program_math_op_err() {
        let mut env = Env::new();
        let e = Expression::MathOp {
            lhs: Box::new(Expression::Boolean(true)),
            op: MathOperator::Divide,
            rhs: Box::new(Expression::Integer(10)),
        };

        let result = interp_program(e, &mut env).unwrap_err();

        assert_eq!(result, InterpErrors::InvalidTypeError);
    }

    #[test]
    fn test_interp_program_bool_op_eq() {
        let mut env = Env::new();
        let e = Expression::BooleanOp {
            lhs: Box::new(Expression::Boolean(true)),
            op: BooleanOperator::Eqaul,
            rhs: Box::new(Expression::Boolean(false)),
        };

        let result = interp_program(e, &mut env).unwrap();

        assert_eq!(result, Types::Boolean(false));
    }

    #[test]
    fn test_interp_program_bool_op_le() {
        let mut env = Env::new();
        let e = Expression::BooleanOp {
            lhs: Box::new(Expression::Integer(1)),
            op: BooleanOperator::LessThan,
            rhs: Box::new(Expression::Integer(10)),
        };

        let result = interp_program(e, &mut env).unwrap();

        assert_eq!(result, Types::Boolean(true));
    }

    #[test]
    fn test_interp_program_bool_op_leq() {
        let mut env = Env::new();
        let e = Expression::BooleanOp {
            lhs: Box::new(Expression::Integer(1)),
            op: BooleanOperator::LessThanEqual,
            rhs: Box::new(Expression::Integer(10)),
        };

        let result = interp_program(e, &mut env).unwrap();

        assert_eq!(result, Types::Boolean(true));
    }

    #[test]
    fn test_interp_program_bool_op_ge() {
        let mut env = Env::new();
        let e = Expression::BooleanOp {
            lhs: Box::new(Expression::Integer(1)),
            op: BooleanOperator::GreaterThan,
            rhs: Box::new(Expression::Integer(10)),
        };

        let result = interp_program(e, &mut env).unwrap();

        assert_eq!(result, Types::Boolean(false));
    }

    #[test]
    fn test_interp_program_bool_op_geq() {
        let mut env = Env::new();
        let e = Expression::BooleanOp {
            lhs: Box::new(Expression::Integer(1)),
            op: BooleanOperator::GreaterThanEqual,
            rhs: Box::new(Expression::Integer(10)),
        };

        let result = interp_program(e, &mut env).unwrap();

        assert_eq!(result, Types::Boolean(false));
    }

    #[test]
    fn test_interp_program_bool_op_mismatch_type_err() {
        let mut env = Env::new();
        let e = Expression::BooleanOp {
            lhs: Box::new(Expression::Boolean(true)),
            op: BooleanOperator::GreaterThanEqual,
            rhs: Box::new(Expression::Integer(10)),
        };

        let result = interp_program(e, &mut env).unwrap_err();

        assert_eq!(result, InterpErrors::MismatchedTypeError);
    }
}
