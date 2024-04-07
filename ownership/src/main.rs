fn main() {}

// 不能直接返回一部分的字符串，但是可以返回查找字符串后的下标
fn first_word(s: &String) -> usize {
    // s 接受的是一个引用，因此 s 的所有权被转移给了函数
    // 首先把字符串转换成字节数组
    let bytes = s.as_bytes();

    // 然后遍历数组，转化为元组
    for (i, &item) in bytes.iter().enumerate() {
        // 这块的 b 不知道什么意思
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
