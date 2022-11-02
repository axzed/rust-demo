fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("the length of s is: {}", len);

    change(&mut s1);

    println!("The changed string is: {}", s1);

    let mut s2 = String::from("vex");

    // it can have only one mutable yy in the same scope
    let r1 = &mut s2;
    // let r2 = &mut s2;
    println!("{}", r1);
}

fn calculate_length(s: &String) -> usize {
    s.len() 
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
