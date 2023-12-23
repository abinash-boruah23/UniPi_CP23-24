use std::cmp;

struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut maxi = i32::MIN;
        let mut sum = 0;

        for &num in nums.iter() {
            sum += num;
            maxi = cmp::max(sum, maxi);
            if sum < 0 {
                sum = 0;
            }
        }

        maxi
    }
}

fn main() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let result = Solution::max_sub_array(nums);
    println!("Maximum Subarray Sum: {}", result);
}
