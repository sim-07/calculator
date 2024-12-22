fn precedence(op: &str) -> i32 {
    match op {
        "+" | "-" => 1,
        "*" | "/" => 2,
        "^" => 3,
        _ => 0,
    }
}

pub fn convert_rpn(input: String) -> Vec<String> {

    let mut stack: Vec<String> = Vec::new();
    let mut output: Vec<String> = Vec::new();

    for token in input.trim().split_whitespace() {
        // Divide la stringa per spazi bianchi

        match token {
            "+" | "-" | "*" | "/" | "^" => {
                while let Some(op) = stack.last() { // Scorre stack dall'ultimo elemento finché non è None
                    if precedence(op) >= precedence(token) && token != "^" { // Se l'operatore già in stack ha precedenza uguale o più alta di token lo si sposta in output. Token si mette in ogni caso nello stack
                        output.push(stack.pop().unwrap().to_string());
                    } else {
                        break;
                    }
                }
                stack.push(token.to_string());
            }

            "(" => {
                stack.push(token.to_string())
            }

            ")" => {
                while let Some(op) = stack.pop() {
                    if op == "(" {
                        break;
                    }
                    output.push(op); // Sposto tutto in output finchè non incontro "("
                }
            }

            _ => match token.parse::<f64>() {
                Ok(num) => {
                    output.push(num.to_string());
                }

                Err(_) => {
                    println!("Carattere {} non riconosciuto", token);
                    return output;
                }
            },
        }
    }
    
    while let Some(op) = stack.pop() {
        output.push(op)
    }

    return output;

}