fn reverse_string(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

fn main() {
    let test_string = "Ujjwal";
    println!("Reversed string: {}", reverse_string(test_string));
}
