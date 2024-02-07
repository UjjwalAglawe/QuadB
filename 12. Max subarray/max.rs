fn max_subarray_sum(nums: &[i32]) -> i32 {
    let mut max_sum = nums[0];
    let mut cur_sum = nums[0];

    for &num in &nums[1..] {
        cur_sum = cur_sum.max(num);
        max_sum = max_sum.max(cur_sum);
    }

    max_sum
}

fn main() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("Maximum subarray sum: {}", max_subarray_sum(&nums));
}
