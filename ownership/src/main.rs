// 在 rust 中，对于某一个值。当拥有它的变量离开作用域范围时，内存会立即被释放。

fn main() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("s1: {}, s2: {}", s1, s2);
}
