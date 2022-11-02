fn main() {
    test_string();
    test_bind();
}

fn test_string() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}

fn test_bind() {
    let x = "hello, world";
    let y = x;
    println!("{}, {}", x, y);
}
