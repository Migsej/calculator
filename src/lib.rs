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
}

pub fn eval_string(equation: String) -> Result<f64> {
    let parsed: Vec<Operation> = parser::parse(equation)?; 
    evaluator::evaluate(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_eval() -> Result<()>{
        let equation: String = "1+2+3+6*2 + 3+4".to_string();
        let evaluated = eval_string(equation)?;
        assert_eq!(evaluated , 25.0);
        Ok(())
    }
    #[test]
    fn test_parser() -> Result<()> {
        let equation: String = "99999+9+3+6*2 + 3+4".to_string();
        let evaluated = eval_string(equation)?;
        assert_eq!(evaluated, 100030.0);
        Ok(())
    }
    #[test]
    fn test_paren() -> Result<()> {
        let equation: String = "2*(5+1)".to_string();
        let evaluated = eval_string(equation)?;
        assert_eq!(evaluated, 12.0);
        Ok(())
    }
    #[test]
    fn test_exponent() -> Result<()> {
        let equation: String = "2^3".to_string();
        let evaluated = eval_string(equation)?;
        assert_eq!(evaluated, 8.0);
        Ok(())
    }
    #[test]
    fn test_plus() -> Result<()> {
        let equation: String = "2+2".to_string();
        let evaluated = eval_string(equation)?;
        assert_eq!(evaluated, 4.0);
        Ok(())
    }
    #[test]
    fn test_minus() -> Result<()> {
        let equation: String = "2-2".to_string();
        let evaluated = eval_string(equation)?;
        assert_eq!(evaluated, 0.0);
        Ok(())
    }
    #[test]
    fn test_negative() -> Result<()> {
        let equation: String = "2+(-2)".to_string();
        let evaluated = eval_string(equation)?;
        assert_eq!(evaluated, 0.0);
        Ok(())
    }
    #[test]
    fn test_sqrt() -> Result<()> {
        let equation: String = "sqrt (2+2)".to_string();
        let evaluated = eval_string(equation)?;
        assert_eq!(evaluated, 2.0);
        Ok(())
    }

}

