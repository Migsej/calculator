use crate::Operation;
use anyhow::{bail, Context, Result};

#[derive(Clone)]
struct Operator{
    equation: Vec<Operation>,
}

impl Operator {
    fn operatesingle<F>(mut self, operation: Operation, func: F) -> Result<Operator> 
                where F: Fn(f64) -> f64 {
        while let Some(index) = self.equation.iter().rposition(|x| x == &operation) {
            let operand = self.equation.get(index+1).context("do it right")?;
            if let Operation::Number(a) = operand {
                self.equation.splice((index)..=(index+1), vec![Operation::Number(func(*a))]);
            } else {
                bail!("something when wrong calculating")
            }
        }
        return Ok(self)
    }

    fn operate<F>(mut self, operation: Operation, func: F) -> Result<Operator>
                where F: Fn(f64, f64) -> f64 {
        while let Some(index) = self.equation.iter().rposition(|x| x == &operation) {
            if  index == 0 {
                bail!("cant operate on first element")
            }
            let firstoperand = self.equation.get(index -1).context("do it right")?;
            let secondoperand = self.equation.get(index+1).context("do it right")?;
            if let (Operation::Number(a), Operation::Number(b)) = (firstoperand,secondoperand) {
                self.equation.splice((index-1)..=(index+1), vec![Operation::Number(func(*a,*b))]);
            } else {
                bail!("something when wrong calculating")
            }
        }
        return Ok(self)
    }
    /// should make a number followed by a negative number be number minus number
    fn fixminus(mut self) -> Operator {
        for (i, v) in self.equation.clone().iter().enumerate() {
            if let Operation::Number(num) = v {
                if i == 0 {
                    //pass
                }else if let Operation::Number(_) = self.clone().equation[i-1] {
                    if num < &0.0 {
                        self.equation.splice(i..=i, vec![Operation::Minus, Operation::Number(num*-1.0)]);
                    }

                }
            }
        }
        return self.clone()
    }
}
fn evaluate_sequence(equation: Vec<Operation>) -> Result<Operation>{
    let operator = Operator{equation};
    let result = operator
        .fixminus()
        .operatesingle(Operation::Sqrt, |a| a.sqrt() )?
        .operatesingle(Operation::Cos, |a| a.cos() )?
        .operatesingle(Operation::Sin, |a| a.sin() )?
        .operate(Operation::Exponent,|a, b| a.powf(b) )?
        .operate(Operation::Multiply,|a, b| a*b )?
        .operate(Operation::Divide ,|a, b| a/b )?
        .operate(Operation::Plus ,|a, b| a+b )?
        .operate(Operation::Minus ,|a, b| a-b )?;

    if result.equation.len() == 1 {
        return Ok(result.equation[0].clone());
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
   if let Operation::Number(n) = evaluate_sequence(equation)? {
        return Ok(n);
   } else {
        bail!("couldnt evaluate string")
   }


}

