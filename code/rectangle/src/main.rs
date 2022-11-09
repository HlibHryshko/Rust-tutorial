// initializing new structure for rectangle dimensions
// implementing debug trait for showing the instances of Rectangle type in the console
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// methods of the Rectangle structure
impl Rectangle {
    // function for calculating the area
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    // setting rectangle dimensions
    let rectangle1 = Rectangle {
        width: 30,
        height: 50,
    };

    // showing the dimensions of the rectangle in the console
    println!(
        "The dimensions of the rectangle are {:#?}",
        rectangle1
    );

    // showing the area in the console
    println!(
        "The area of the rectangle is {} square pixels",
        rectangle1.area()
    );
}