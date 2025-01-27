// 创建一个变量
// 创建一个随机数
// 将用户输入的数字赋值给变量
// 循环将变量与随机数进行比较
// 正确则退出循环，否则继续循环
// 输出结果给用户
use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数游戏!!!");
    println!("请输入一个数字:");

    let rng: u32 = thread_rng().gen_range(0..101);

    loop {
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取数字");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&rng) {
            Ordering::Less => println!("你猜小了!"),
            Ordering::Greater => println!("你猜大了!"),
            Ordering::Equal => {
                println!("你猜对了!");
                break;
            }
        }
    }
}
