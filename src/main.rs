
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


fn main() {
    let equation: String = "3 * 3 + 5 + 2 * 2 + 3 - 3 * 5".to_string();

    let parsed: Vec<Operation> = equation.split_whitespace()
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
                .collect();
    let multiplied = operate(parsed, Operation::Multiply,|a, b| a*b ).unwrap();
    let divided = operate(multiplied, Operation::Divide ,|a, b| a/b ).unwrap();
    let plussed = operate(divided, Operation::Plus ,|a, b| a+b ).unwrap();
    let minussed = operate(plussed, Operation::Minus ,|a, b| a-b ).unwrap();
    
    println!("{:?}", minussed);
}
