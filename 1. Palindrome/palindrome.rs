fn main() {
    let mut input = String::new();
    println!("Enter a string: ");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let input = input.trim(); // Trim leading/trailing whitespace

    if is_palindrome(input) {
        println!("{} is a palindrome", input);
    } else {
        println!("{} is not a palindrome", input);
    }
}

fn is_palindrome(input: &str) -> bool {
    let chars: Vec<char> = input.chars().collect();

    let mut i = 0;
    let mut j = chars.len() - 1;

    while i < j {
        if chars[i] != chars[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }

    true
}
