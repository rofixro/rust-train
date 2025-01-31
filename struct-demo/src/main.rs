// struct 一旦声明是可变的，那么其中所有的变量都是可变的

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    println!("Hello, world!");

    let user1 = User {
        active: true,
        sign_in_count: 555,
        username: String::from("Rofix"),
        email: String::from("abc@gmail.com"),
    };

    let user2  = User {
        active: false,
        sign_in_count: 666,
        ..user1
    }
}

// tuple struct
struct Color(i32, i32, i32);

let black = Color(0, 0, 0);
