fn main() {
    let number: u32 = "142".parse().expect("Not a number");
    println!("String to Int parsing: {}", number - 100);

    // let number = "142".parse().expect("Not a number");
    // println!("{}", number - 100 );

    let x = 123.456; // f64
    let y: f32 = 2.0; // f32
    println!("{} and {}", x, y);

    let is_true = true;
    println!("{}", is_true);

    let z_1 = 'Z';
    let z_2 = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("{} {} {}", z_1, z_2, heart_eyed_cat);
}
