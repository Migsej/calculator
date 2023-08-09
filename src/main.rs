
#[derive(Debug, PartialEq, Copy, Clone)]
enum Operation{
    Number(f64),
    Plus,
    Minus,
    Divide,
    Multiply,
}

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

fn evaluate(equation: Vec<Operation>) -> Option<f64> {
    let multiplied = operate(equation, Operation::Multiply,|a, b| a*b )?;
    let divided = operate(multiplied, Operation::Divide ,|a, b| a/b )?;
    let plussed = operate(divided, Operation::Plus ,|a, b| a+b )?;
    let minussed = operate(plussed, Operation::Minus ,|a, b| a-b )?;
    if minussed.len() == 1 {
        if let Operation::Number(n) = minussed[0] {
            return Some(n)
        }
    }
    return None
}

fn parse(equation: String) -> Vec<Operation> {
    equation.split_whitespace()
        .map(|x| {
            match x.parse::<f64>() {
                Ok(n) => Some(Operation::Number(n)),
                Err(_s) => { match x {
                                "+" => Some(Operation::Plus),
                                "*" => Some(Operation::Multiply),
                                "/" => Some(Operation::Divide),
                                "-" => Some(Operation::Minus),
                                _ => None,
                            }}
                    }
            }).filter(|x| x.is_some()).map(|x| x.unwrap())
                .collect()
}


fn main() {
    let equation: String = "3 * 3 + 5 + 2 * 2 + 3 - 3 * 5 / 1231231 + 44".to_string();

    let parsed: Vec<Operation> = parse(equation); 
    let evaluated = evaluate(parsed);
    match evaluated {
        Some(n) => println!("{}", n),
        None => println!("Error"),
    }

    
}
