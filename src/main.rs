use std::io;
use std::time::Instant;

mod rpn;
fn main() {
    
    let mut input = String::new();

    println!("Espressione: ");
    io::stdin().read_line(&mut input).unwrap();

    let start = Instant::now();

    let rpn_expression = rpn::convert_rpn(input);
    //println!("{:?}", rpn_expression);

    let mut stack: Vec<f64> = Vec::new();
    for el in rpn_expression {
        if let Ok(_) = el.parse::<i32>() {
            stack.push(el.parse().unwrap());
        } else {
            let n2: f64 = stack.pop().unwrap();
            let n1: f64 = stack.pop().unwrap();

            let result = match el.as_str() {
                "+" => n1 + n2,
                "-" => n1 - n2,
                "*" => n1 * n2,
                "/" => n1 / n2,
                "^" => n1.powf(n2),
                _ => break
            };
            stack.push(result);
            
        }
    }

    let duration = start.elapsed();
    println!("Tempo impiegato: {} ns", duration.as_nanos());
    println!("{:?}", stack[0].to_string());
}
