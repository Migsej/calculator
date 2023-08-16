use crate::Operation;
use anyhow::{bail, Context, Result};


fn operatesingle<F>(mut equation: Vec<Operation>, operation: Operation, func: F) -> Result<Vec<Operation>> 
            where F: Fn(f64) -> f64 {
    while let Some(index) = equation.iter().rposition(|x| x == &operation) {
        let operand = equation.get(index+1).context("do it right")?;
        if let Operation::Number(a) = operand {
            equation.splice((index)..=(index+1), vec![Operation::Number(func(*a))]);
        } else {
            bail!("something when wrong calculating")
        }
    }
    return Ok(equation)
}

fn operate<F>(mut equation: Vec<Operation>, operation: Operation, func: F) -> Result<Vec<Operation>>
            where F: Fn(f64, f64) -> f64 {
    while let Some(index) = equation.iter().rposition(|x| x == &operation) {
        let firstoperand = equation.get(index-1).context("do it right")?;
        let secondoperand = equation.get(index+1).context("do it right")?;
        if let (Operation::Number(a), Operation::Number(b)) = (firstoperand,secondoperand) {
            equation.splice((index-1)..=(index+1), vec![Operation::Number(func(*a,*b))]);
        } else {
            bail!("something when wrong calculating")
        }
    }
    return Ok(equation)
}

fn evaluate_sequence(equation: Vec<Operation>) -> Result<Operation>{
    let squirted = operatesingle(equation, Operation::Sqrt, |a| a.sqrt() )?;
    let exponented = operate(squirted, Operation::Exponent,|a, b| a.powf(b) )?;
    let multiplied = operate(exponented, Operation::Multiply,|a, b| a*b )?;
    let divided = operate(multiplied, Operation::Divide ,|a, b| a/b )?;
    let plussed = operate(divided, Operation::Plus ,|a, b| a+b )?;
    let minussed = operate(plussed, Operation::Minus ,|a, b| a-b )?;
    if minussed.len() == 1 {
        return Ok(minussed[0].clone());
    }
    bail!("couldnt evaluate sequence")
}

pub fn evaluate(mut equation: Vec<Operation>) ->  Result<f64>{
    let mut open_parenthesis = 0;
    let mut closed_parenthesis;
    let mut index = 0;
    while equation.contains(&Operation::OpenParenthesis) {
        while equation.get(index).context("ya parenthesis is wrong mate")? != &Operation::ClosedParenthesis {
            if equation[index] == Operation::OpenParenthesis {
                open_parenthesis = index;
            }
            index += 1;
        }
        closed_parenthesis = index;
        let evalparen = evaluate_sequence(equation[(open_parenthesis+1)..=(closed_parenthesis - 1)].to_vec());
        equation.splice(open_parenthesis..=closed_parenthesis, vec![evalparen.unwrap()]);
        index = 0;
    }
    if let Ok(Operation::Number(n)) = evaluate_sequence(equation) {
        return Ok(n);
    }
    bail!("couldnt evaluate string") 

}

