// 要了解所有权机制，首先要了解栈内存和堆内存的概念。
// rust 的核心就是所有权机制，也就是如何管理内存的方式
// 在 rust 中，对于某一个值。当拥有它的变量离开作用域范围时，内存会立即被释放。
// 所有权系统并没有减慢程序的运行速度，因为它是发生在编译阶段，不会影响运行时的性能。
// 管理 heap 的数据事所有权系统存在的原因

// 所有权规则
// 1. 每个值都有一个变量，这个变量是这个值的所有者
// 2. 每个值同时只能有一个所有者
// 3. 当所有者超出作用域时，该值将被删除
// 引用和借用
// 引用的规则（只能存在下面一种）
// 1. 一个可变的引用
// 2. 任意数量的不可变的引用
// 不持有所有权的数据类型：切片
fn main() {
    let a = 5;
    let b = a;

    println!("{}, {}", a, b);

    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}, {}", s1, s2);

    let s3 = String::from("hello");
    let s4 = s3.clone();

    println!("{}, {}", s3, s4);

    let c = 6;
    let s5 = String::from("hello");

    makes_copy(c);
    take_ownership(s5);

    println!("c: {}", c);

    let s6 = gives_ownership();
    let s7 = String::from("hello");
    let s8 = takes_and_gives_back(s7);
    let mut s9 = String::from("hello");
    let (len, s10) = calculate_length(&mut s9);
    // let d = dangle();
    println!("{}, {}", s9, len);

    let s11 = String::from("hello world");
    let index = first_word(&s11);

    println!("{}", index)
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_number: i32) {
    println!("{}", some_number);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &mut String) -> (usize, &mut String) {
    s.push_str(", world");
    (s.len(), s)
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
