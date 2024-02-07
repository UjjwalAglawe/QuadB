fn median(arr: &[i32]) -> f64 {
    let mid = arr.len() / 2;
    if arr.len() % 2 == 0 {
        (arr[mid - 1] + arr[mid]) as f64 / 2.0
    } else {
        arr[mid] as f64
    }
}

fn main() {
    let sorted_array = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("Median: {}", median(&sorted_array));
}
