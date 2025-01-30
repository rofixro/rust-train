// 在 rust 中，对于某一个值。当拥有它的变量离开作用域范围时，内存会立即被释放。

fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}
