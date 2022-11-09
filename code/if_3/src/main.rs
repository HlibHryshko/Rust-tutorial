fn main() {
    let condition = 0 < 1;
    let number1 = if condition {12} else {5};
    let condition = -1 > 0;
    let number2 = if condition {12} else {5};
    println!("{}", number1);
    println!("{}", number2);
}
