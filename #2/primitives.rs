fn main() {
    // 给出类型标注
    let logical: bool = ture;

    // 常规标注
    let a_float: f64 = 1.0;
    // 后缀说明
    let an_integer = 5i32;

    // 不写类型会自动判断
    let default_float = 3.0; // f64
    let default_integer = 7; // i32

    // 类型还可以根据上下文自动推断
    let mut inferred_type = 12; // 根据下一行的赋值推断为 i64
    inferred_type = 4294967296i64;

    // 可变的 （mutable）变量，其值可以改变
    let mut mutable = 12; // Mutable i32
    mutable = 21;

    // 会报错，变量的类型并不能改变
    mutable = true;

    // 但是可以遮蔽（shadow）前面的变量
    let mutable = true;
}
