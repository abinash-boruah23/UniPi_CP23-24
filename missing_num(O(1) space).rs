struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let exp_sum = (n * (n + 1)) / 2;
        let sum: i32 = nums.iter().sum();
        exp_sum - sum
    }
}

fn main() {
    let nums = vec![9,6,4,2,3,5,7,0,1];
    let result = Solution::missing_number(nums);
    println!("Missing Number: {}", result);
}
