use crate::Operation;

pub fn parse(equation: String) -> Result<Vec<Operation>, &'static str> {
    let split = equation 
        .chars()
        .map(|x| {
            match x {
                '(' => " ( ".to_string(),
                ')' => " ) ".to_string(),
                '+' => " + ".to_string(),
                '*' => " * ".to_string(),
                '/' => " / ".to_string(),
                '^' => " ^ ".to_string(),
                '-' => " - ".to_string(),
                's' => " s ".to_string(),
                _   => x.to_string(),
            }
        })
        .collect::<String>();

    let parsed: Vec<Option<Operation>> = split
        .split_whitespace()
        .map(|x| {
            match x.parse::<f64>() {
                Ok(n) => Some(Operation::Number(n)),
                Err(_s) => { match x {
                                "+" => Some(Operation::Plus),
                                "^" => Some(Operation::Exponent),
                                "*" => Some(Operation::Multiply),
                                "/" => Some(Operation::Divide),
                                "-" => Some(Operation::Minus),
                                "(" => Some(Operation::OpenParenthesis),
                                ")" => Some(Operation::ClosedParenthesis),
                                "s" => Some(Operation::Sqrt),
                                _ => None,
                            }}
                    }
            }).collect();

        let filtered: Vec<Operation> = parsed.iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();

        if parsed.len() == filtered.len() {
            return Ok(filtered)
        }
        return Err("couldnt parse string")
}
