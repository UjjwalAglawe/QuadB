fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > 0 && k <= arr.len() {
        let mut sorted_arr = arr.to_vec();
        sorted_arr.sort();
        Some(sorted_arr[k - 1])
    } else {
        None
    }
}

fn main() {
    let test_array = vec![5, 2, 8, 1, 9, 3, 7];
    let k = 3;
    println!("{}th smallest element: {:?}", k, kth_smallest(&test_array, k));
}
