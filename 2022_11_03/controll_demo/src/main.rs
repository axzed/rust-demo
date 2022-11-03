fn main() {
    let a = [1, 2, 3, 4];
    // `.iter()`方法把`a`数组变成一个迭代器
    for (i, v) in a.iter().enumerate() {
        println!("{} {}", i, v);
    }

    for i in 1..=4 {
        if i == 2 {
            continue;
        }
        println!("{}", i);
    }

    let mut n = 0;
    while n < 5 {
        println!("{}!", n);
        n += 1;
    }
    println!("我出来了!");
}
