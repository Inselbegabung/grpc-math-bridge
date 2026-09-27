use clap::Parser;
use protocol::{Command, CommandData};
use regex::Regex;

#[derive(Debug, Parser)]
#[command(version, about = "CLI client for the gRPC math bridge daemon")]
pub struct Args {
    /// Path to the daemon Unix socket
    #[arg(long, default_value = "/tmp/math.sock")]
    pub socket_path: String,

    /// Math expression to evaluate like "1+1"
    expression: String,
}

impl Args {
    pub fn command(&self) -> Result<Command, String> {
        parse_expression(&self.expression)
    }
}

fn parse_expression(expression: &str) -> Result<Command, String> {
    let expression: String = expression.chars().filter(|c| !c.is_whitespace()).collect();

    let regex =
        Regex::new(r"^(-?\d+(?:\.\d+)?)([+\-*/])(-?\d+(?:\.\d+)?)$").map_err(|e| e.to_string())?;

    let captures = regex
        .captures(&expression)
        .ok_or_else(|| format!("Invalid expression: '{}'", expression))?;

    let lhs = captures[1].parse::<f64>().map_err(|e| e.to_string())?;
    let rhs = captures[3].parse::<f64>().map_err(|e| e.to_string())?;

    let data = CommandData { lhs, rhs };

    match &captures[2] {
        "+" => Ok(Command::Addition(data)),
        "-" => Ok(Command::Subtraction(data)),
        "*" => Ok(Command::Multiplication(data)),
        "/" => Ok(Command::Division(data)),
        operator => Err(format!("Unsupported operator: '{operator}'")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_addition() {
        let command = parse_expression("1+1").expect("valid expression");

        assert_eq!(
            command,
            Command::Addition(CommandData { lhs: 1.0, rhs: 1.0 })
        );
    }

    #[test]
    fn parse_subtraction() {
        let command = parse_expression("10-5").expect("valid expression");

        assert_eq!(
            command,
            Command::Subtraction(CommandData {
                lhs: 10.0,
                rhs: 5.0,
            })
        );
    }

    #[test]
    fn parse_multiplication() {
        let command = parse_expression("3*4").expect("valid expression");

        assert_eq!(
            command,
            Command::Multiplication(CommandData { lhs: 3.0, rhs: 4.0 })
        );
    }

    #[test]
    fn parse_division() {
        let command = parse_expression("10/2").expect("valid expression");

        assert_eq!(
            command,
            Command::Division(CommandData {
                lhs: 10.0,
                rhs: 2.0,
            })
        );
    }

    #[test]
    fn parse_with_whitespace() {
        let command = parse_expression(" 10.5 + 2.5 ").expect("valid expression");

        assert_eq!(
            command,
            Command::Addition(CommandData {
                lhs: 10.5,
                rhs: 2.5,
            })
        );
    }

    #[test]
    fn parse_negative_numbers() {
        let command = parse_expression("-10.5+-2.5").expect("valid expression");

        assert_eq!(
            command,
            Command::Addition(CommandData {
                lhs: -10.5,
                rhs: -2.5,
            })
        );
    }

    #[test]
    fn reject_invalid_expression() {
        let result = parse_expression("invalid");

        assert!(result.is_err());
    }

    #[test]
    fn reject_unsupported_operator() {
        let result = parse_expression("10%2");

        assert!(result.is_err());
    }
}
