fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let mut prefix = String::new();

    for (i, c) in strings[0].chars().enumerate() {
        if strings.iter().all(|s| s.chars().nth(i) == Some(c)) {
            prefix.push(c);
        } else {
            break;
        }
    }
    prefix
}

fn main() {
    let test_strings = vec![
        String::from("apple"),
        String::from("app"),
        String::from("apricot"),
    ];
    println!("Longest common prefix: {}", longest_common_prefix(&test_strings));
}
