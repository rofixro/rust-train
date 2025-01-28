const MAX_POINTS: u32 = 100_000; // constant

fn main() {
    let x = 5;
    let x = x + 1; // shadowing

    let spaces = "    ";
    let spaces = spaces.len();

    println!("this is the value of spaces: {} {} {}", spaces, x, MAX_POINTS);
}