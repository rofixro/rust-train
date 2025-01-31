// struct 一旦声明是可变的，那么其中所有的变量都是可变的

// tuple struct
// struct Color(i32, i32, i32);

// let black = Color(0, 0, 0);

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// impl 关联的方法可以这样写,以便更好的组织代码
struct Rectangle {
    width: u32,
    length: u32,
}

// 方法
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }
    // 关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            length: size,
        }
    }
}

fn main() {
    println!("Hello, world!");

    let user1 = User {
        active: true,
        sign_in_count: 555,
        username: String::from("Rofix"),
        email: String::from("abc@gmail.com"),
    };

    let user2 = User {
        active: false,
        sign_in_count: 666,
        ..user1
    };

    let rect = Rectangle {
        width: 30,
        length: 50,
    };

    let s = Rectangle::square(20);

    println!("{}", rect.area());
}
