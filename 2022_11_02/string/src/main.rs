fn main() {
    let my_name = String::from("Pascal");
    greet(my_name);

    let s = String::from("hello world!");

    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str());

    let mut s1 = String::from("Hello ");
    s1.push('r');
    println!("{}", s1);

    s1.push_str("ust!");
    println!("{}", s1);

    let mut s2 = String::from("Hello rust!");
    s2.insert(5, ',');
    println!("{}", s2);

    s2.insert_str(6, "I like");
    println!("{}", s2);

    for c in "中国人".chars() {
        println!("{}", c);
    }
}

fn say_hello(s: &str) {
    println!("{}", s);
}


fn greet(name: String) {
    println!("Hello, {}!", name);
}