fn main() {
    println!("Hello, world!");
    println!("I'm a Rustacean!");
    // 多种注释方式
    // 这里是单行注释
    /*
     * 这里是多行注释
     * 可以在里面随便编写
     * 前面的添加的 * 不是必须的，只是一种编程风格
     */
    // 多行注释可以插在表达式里面
    let x = 1 + /* 3 */  4;
    println!("Is `x` 5 or 7? x = {}", x);
}
