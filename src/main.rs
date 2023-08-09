
#[derive(Debug, PartialEq, Copy, Clone)]
enum Operation{
    Number(f64),
    Plus,
    Minus,
    Divide,
    Multiply,
    OpenParenthesis,
    ClosedParenthesis,
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

fn evaluate(mut equation: Vec<Operation>) -> Option<f64> {
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

fn parse(equation: String) -> Vec<Operation> {
    equation
        .chars()
        .map(|x| {
            match x {
                '(' => " ( ".to_string(),
                ')' => " ) ".to_string(),
                '+' => " + ".to_string(),
                '*' => " * ".to_string(),
                '/' => " / ".to_string(),
                '-' => " - ".to_string(),
                _ => x.to_string(),
            }
        })
        .collect::<String>()
        .split_whitespace()
        .map(|x| {
            match x.parse::<f64>() {
                Ok(n) => Some(Operation::Number(n)),
                Err(_s) => { match x {
                                "+" => Some(Operation::Plus),
                                "*" => Some(Operation::Multiply),
                                "/" => Some(Operation::Divide),
                                "-" => Some(Operation::Minus),
                                "(" => Some(Operation::OpenParenthesis),
                                ")" => Some(Operation::ClosedParenthesis),
                                _ => None,
                            }}
                    }
            }).filter(|x| x.is_some()).map(|x| x.unwrap())
                .collect()
}


fn main() {
    let equation: String = "1+(2+3+(6*2) + (3+4))".to_string();

    let parsed: Vec<Operation> = parse(equation); 
    let evaluated = evaluate(parsed);
    match evaluated {
        Some(n) => println!("{}", n),
        None => println!("Error"),
    }

    
}
