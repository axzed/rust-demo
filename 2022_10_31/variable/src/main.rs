fn main() {
    var_mut();
    ignore_var();
    var_dec();
    var_shadowing();
    println!("Hello, world!");
}

// var_mut 变量可变性
fn var_mut() {
    let mut x = 5; 
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn ignore_var() {
    let _x = 5;
    let _y = 10;
}

fn var_dec() {
    let (a, mut b): (bool, bool) = (true, false);
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);
}

fn var_shadowing() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x)
    }

    println!("The value of x is: {}", x);
}
