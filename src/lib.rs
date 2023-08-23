use anyhow::Result;

pub mod parser;
pub mod evaluator;


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Operation{
    Number(f64),
    Plus,
    Minus,
    Divide,
    Multiply,
    Exponent,
    OpenParenthesis,
    ClosedParenthesis,
    Sqrt,
    Cos,
    Sin,
}

pub fn eval_string(equation: String) -> Result<f64> {
    let parsed: Vec<Operation> = parser::parse(equation)?; 
    evaluator::evaluate(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_eval {
        ($name:ident, $equation:expr, $expected:expr) => {
            #[test]
            fn $name() -> Result<()> {
                let evaluated = eval_string($equation.to_string())?;
                assert_eq!(evaluated, $expected);
                Ok(())
            }
        };
    }
    test_eval!(test_eval, "1+2+3+6*2 + 3+4" , 25.0);
    test_eval!(test_parser, "99999+9+3+6*2 + 3+4", 100030.0);
    test_eval!(test_paren, "2*(5+1)", 12.0);
    test_eval!(test_exponent, "2^3", 8.0);
    test_eval!(test_plus, "2+2", 4.0);
    test_eval!(test_minus, "2-2", 0.0);
    test_eval!(test_negative, "2+(-2)", 0.0);
    test_eval!(test_sqrt, "sqrt (2+2)", 2.0);
}

