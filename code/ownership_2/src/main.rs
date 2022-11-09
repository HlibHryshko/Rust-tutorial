fn main() {
    let mut s = String::from("hello");
    println!("{}", s);
    append_world(&mut s);
    println!("{}", s);
}

fn append_world(s: &mut String) {
    s.push_str(", world!");
}
