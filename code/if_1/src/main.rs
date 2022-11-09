fn main() {
    let number1 = 3;
    let number2 = 5;
    check_number(number1);
    check_number(number2);
}

fn check_number(number: i32) {
    if number < 5 {
        println!("{}", number);
    } else {
        println!("Error");
    }
}
