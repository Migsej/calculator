use crate::Operation;
use pest::Parser;
use pest_derive::Parser;
use anyhow::Result;

#[derive(Parser)]
#[grammar = "parser.pest"] 
struct EquationParser;

pub fn parse(equation: String) -> Result<Vec<Operation>> {
    let parsedequ = EquationParser::parse(Rule::equation, &equation)?.next().unwrap();
    
    let mut result = Vec::new();

    for iden in parsedequ.into_inner() {
        let bla = match iden.as_rule() {
            Rule::number => {
                let number = iden
                    .as_str()
                    .parse::<f64>()?;
                Operation::Number(number)
            },
            Rule::plus => Operation::Plus,
            Rule::minus => Operation::Minus,
            Rule::multiply => Operation::Multiply,
            Rule::divide => Operation::Divide,
            Rule::sqrt => Operation::Sqrt,
            Rule::exponent => Operation::Exponent,
            Rule::openparen => Operation::OpenParenthesis,
            Rule::closedparen => Operation::ClosedParenthesis,
            _ => unreachable!(),
        };
        result.push(bla);
    }
    Ok(result)
}
