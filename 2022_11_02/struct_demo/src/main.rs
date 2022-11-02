#[derive(Debug)]
struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut user1 = User {
        email: String::from("vex@qq.com"),
        username: String::from("vex"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("axzed@gmail.com");

    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    let f1_name = &f1.name;
    let f1_length = &f1.data.len();
    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);

    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{:?}", rec1);
}
