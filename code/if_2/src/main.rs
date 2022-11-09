fn main() {
    check_grade(2);
    check_grade(11);
    check_grade(9);
    check_grade(999);
}

fn check_grade(grade: i32) {
    // checking if the grade is in the correct range
    if grade <= 0 || grade > 11 {
        println!("Error! The grade is wrong! It must be in range 1-11");
    } else if grade <= 4 {
        println!("Primary school");
    } else if grade <= 9 {
        println!("Secondary school");
    } else {
        println!("High school");
    }
}
