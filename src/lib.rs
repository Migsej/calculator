pub mod parser;
pub mod evaluator;


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Operation{
    Number(f64),
    Plus,
    Minus,
    Divide,
    Multiply,
    OpenParenthesis,
    ClosedParenthesis,
}

pub fn eval_string(equation: String) -> Option<f64> {
    let parsed: Vec<Operation> = parser::parse(equation); 
    evaluator::evaluate(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_eval() {
        let equation: String = "1+2+3+6*2 + 3+4".to_string();
        let evaluated = eval_string(equation);
        assert_eq!(evaluated, Some(25.0));
    }
    #[test]
    fn test_parser() {
        let equation: String = "99999+9+3+6*2 + 3+4".to_string();
        let evaluated = eval_string(equation);
        assert_eq!(evaluated, Some(100030.0));
    }
    #[test]
    fn test_paren() {
        let equation: String = "2*(5+1)".to_string();
        let evaluated = eval_string(equation);
        assert_eq!(evaluated, Some(12.0));
    }
}
