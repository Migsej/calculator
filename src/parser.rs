use crate::Operation;

pub fn parse(equation: String) -> Vec<Operation> {
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
