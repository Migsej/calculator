use crate::Operation;

fn operate<F>(mut equation: Vec<Operation>, operation: Operation, func: F) -> Option<Vec<Operation>> 
            where F: Fn(f64, f64) -> f64 {
    while let Some(index) = equation.iter().rposition(|x| x == &operation) {
        if let (Operation::Number(a), Operation::Number(b)) = (equation[index-1], equation[index+1]) {
            equation.splice((index-1)..(index+2), vec![Operation::Number(func(a,b))]);
        } else {
            return None
        }
    }
    return Some(equation)
}

fn evaluate_sequence(equation: Vec<Operation>) -> Option<Operation> {
    let multiplied = operate(equation, Operation::Multiply,|a, b| a*b )?;
    let divided = operate(multiplied, Operation::Divide ,|a, b| a/b )?;
    let plussed = operate(divided, Operation::Plus ,|a, b| a+b )?;
    let minussed = operate(plussed, Operation::Minus ,|a, b| a-b )?;
    if minussed.len() == 1 {
        return Some(minussed[0].clone());
    }
    return None
}

pub fn evaluate(mut equation: Vec<Operation>) -> Option<f64> {
    let mut open_parenthesis = 0;
    let mut closed_parenthesis = 0;
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
    if let Some(Operation::Number(n)) = evaluate_sequence(equation) {
        return Some(n);
    }
    return None

}

