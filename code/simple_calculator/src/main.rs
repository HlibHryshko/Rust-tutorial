use std::io;

fn read()-> String {
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");
    
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }

    input
}

fn calculator(){
    let number_1 = read().parse::<f64>().unwrap();
    let number_2 = read().parse::<f64>().unwrap();  
    let sign: Vec<char> = read().chars().collect();
    let sign = sign[0];
    
    let mut result: f64 = 0.;
    match sign {
        '+' => result = number_1 + number_2,
        '-' => result = number_1 - number_2,
        '*' => result = number_1 * number_2,
        '/' => result = number_1 / number_2,
        _ => println!("Incorrect symbol {}", sign)
    }
    println!("{} {} {} = {}", number_1, sign, number_2, result);
}
fn main() {
    loop {
        calculator();
    }
}
