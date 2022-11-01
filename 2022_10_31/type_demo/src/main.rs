fn main() {
    // let x = (-42.0_f32).sqrt();
    // if x.is_nan() {
    //     println!("未定义的数学行为")
    // }
    ex_01();
    ex_02();
}

fn ex_01() {
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
    ];

    println!("{:.2}", forty_twos[0]);
}

// range
fn ex_02() {
    for i in 1..=5 {
        println!("{}", i);
    }
    for i in 'a'..='z' {
        println!("{}", i);
    }
}

