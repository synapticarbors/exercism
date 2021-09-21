#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

fn pop_two(stack: &mut Vec<i32>) -> Option<(i32, i32)> {
    match (stack.pop(), stack.pop()) {
        (Some(a), Some(b)) => Some((a, b)),
        _ => None,
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = vec![];

    for x in inputs {
        match x {
            CalculatorInput::Add => {
                if let Some((a, b)) = pop_two(&mut stack) {
                    stack.push(b + a)
                }
            }
            CalculatorInput::Subtract => {
                if let Some((a, b)) = pop_two(&mut stack) {
                    stack.push(b - a)
                }
            }
            CalculatorInput::Multiply => {
                if let Some((a, b)) = pop_two(&mut stack) {
                    stack.push(b * a)
                }
            }
            CalculatorInput::Divide => {
                if let Some((a, b)) = pop_two(&mut stack) {
                    stack.push(b / a)
                }
            }
            CalculatorInput::Value(x) => stack.push(*x),
        }
    }

    if stack.len() == 1 {
        Some(stack[0])
    } else {
        None
    }
}
