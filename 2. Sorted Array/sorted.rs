fn occur(arr: &[i32], target: i32) -> Option<usize> {
    for (i, &num) in arr.iter().enumerate() {
        if num == target {
            return Some(i);
        }
    }
    None
}

fn main() {
    let sorted_array = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let target_number = 5;
    match occur(&sorted_array, target_number) {
        Some(index) => println!("Index of first occurrence of {}: {}", target_number, index),
        None => println!("{} does not occur in the array", target_number),
    }
}
