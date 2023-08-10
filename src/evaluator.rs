use crate::Operation;

fn operate<F>(mut equation: Vec<Operation>, operation: Operation, func: F) -> Result<Vec<Operation>, &'static str> 
            where F: Fn(f64, f64) -> f64 {
    while let Some(index) = equation.iter().rposition(|x| x == &operation) {
        if let (Operation::Number(a), Operation::Number(b)) = (equation[index-1], equation[index+1]) {
            equation.splice((index-1)..(index+2), vec![Operation::Number(func(a,b))]);
        } else {
            return Err("something when wrong calculating")
        }
    }
    return Ok(equation)
}

fn evaluate_sequence(equation: Vec<Operation>) -> Result<Operation,  &'static str>{
    let exponented = operate(equation, Operation::Exponent,|a, b| a.powf(b) )?;
    let multiplied = operate(exponented, Operation::Multiply,|a, b| a*b )?;
    let divided = operate(multiplied, Operation::Divide ,|a, b| a/b )?;
    let plussed = operate(divided, Operation::Plus ,|a, b| a+b )?;
    let minussed = operate(plussed, Operation::Minus ,|a, b| a-b )?;
    if minussed.len() == 1 {
        return Ok(minussed[0].clone());
    }
    return Err("couldnt evaluate sequence")
}

pub fn evaluate(mut equation: Vec<Operation>) ->  Result<f64,  &'static str>{
    let mut open_parenthesis = 0;
    let mut closed_parenthesis;
    let mut index = 0;
    while equation.contains(&Operation::OpenParenthesis) {
        while equation[index] != Operation::ClosedParenthesis {
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
    return Err("couldnt evaluate string") 

}

