pub struct WordProblem;

pub fn answer(command: &str) -> Option<i32> {
    if !command.starts_with("What is ") {
        return None;
    }

    let tokens: Vec<String> = command
        .trim_start_matches("What is")
        .trim_end_matches('?')
        .replace("plus", "+")
        .replace("minus", "-")
        .replace("multiplied by", "*")
        .replace("divided by", "/")
        .replace("raised to the", "^")
        .replace("power", "")
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.to_string())
        .collect();

    let mut soln = tokens.get(0)?.parse::<i32>().ok()?;

    for g in tokens[1..].chunks(2) {
        let num = g
            .get(1)?
            .chars()
            .filter(|c| c.is_digit(10) || *c == '-')
            .collect::<String>()
            .parse::<i32>()
            .ok()?;

        match g.get(0)?.as_str() {
            "+" => soln += num,
            "-" => soln -= num,
            "*" => soln *= num,
            "/" => soln /= num,
            "^" => soln = soln.pow(num as u32),
            _ => return None,
        }
    }

    Some(soln)
}
