fn main() {
    {
        let mut s = String::from("hello");

        s.push_str(", world");  

        println!("{}", s);

    }

    {
        let x = 5;
        let y = x;

        println!("{}", y);

        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("{}; {}", s1, s2);
    }
}