use calculator::eval_string;

fn main() {
    let equation: String = "1+2+3+6*2 + 3+4".to_string();
    let evaluated = eval_string(equation);

    match evaluated {
        Some(n) => println!("{}", n),
        None => println!("Error"),
    }

    
}
