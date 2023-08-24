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
        let mut bla = match iden.as_rule() {
            Rule::number => {
                let number = iden
                    .as_str()
                    .parse::<f64>()?;
                vec![Operation::Number(number)]
            },
            Rule::plus => vec![Operation::Plus],
            Rule::minus => vec![Operation::Minus],
            Rule::multiply => vec![Operation::Multiply],
            Rule::divide => vec![Operation::Divide],
            Rule::sqrt => vec![Operation::Sqrt],
            Rule::sin => vec![Operation::Sin],
            Rule::cos => vec![Operation::Cos],
            Rule::raised_number => {
                let uuuh = parse_raised(iden.as_str())?;
                vec![Operation::Exponent, Operation::Number(uuuh)]
            },
            Rule::exponent => vec![Operation::Exponent],
            Rule::openparen => vec![Operation::OpenParenthesis],
            Rule::closedparen => vec![Operation::ClosedParenthesis],
            _ => unreachable!(),
        };
        result.append(&mut bla);
    }
    Ok(result)
}

fn parse_raised(num: &str) -> Result<f64> {
    Ok(num.chars().map(|x| {
        match x {
            '¹' => '1',
            '²' => '2',
            '³' => '3',
            '⁴' => '4',
            '⁵' => '5',
            '⁶' => '6',
            '⁷' => '7',
            '⁸' => '8',
            '⁹' => '9',
            '⁰' => '0',
            _   => x,
        }
        }).collect::<String>().parse::<f64>()?)
}
