fn main() {
    let s1 = String::from("Hello");

    let len = calculate_length(&s1);

    let sb = aaa(&s1);

    println!("The length of '{s1}' is {len}.{sb}");
}

fn aaa(s: &String) -> usize {
    s.len()
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
