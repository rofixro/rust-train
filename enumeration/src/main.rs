// 枚举：用来表示一组固定的选项,是一种自定义的数据类型
enum IpAddrKind {
    V4(u8, u8, u8, u8), // 枚举变体
    V6,
}

fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);

    route(four);
    route(IpAddrKind::V6);
}

fn route(ip_kind: IpAddrKind) {}

// 数据可以附加到枚举的变体中
enum IpAddr {
    V4(String),
    V6(String),
}
