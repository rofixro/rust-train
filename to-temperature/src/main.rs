fn main() {
    let celsius = 36.5;
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("摄氏 {} 度等于华氏 {} 度", celsius, fahrenheit);

    let fahrenheit = 97.7;
    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("华氏 {} 度等于摄氏 {} 度", fahrenheit, celsius);
}

// 从摄氏转化为华氏
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

// 从华氏转化为摄氏
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}
