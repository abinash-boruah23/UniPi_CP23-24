use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut s = HashSet::new();

        for &num in nums.iter() {
            s.insert(num);
        }

        for i in 0..nums.len() {
            if !s.contains(&(i as i32)) {
                return i as i32;
            }
        }

        nums.len() as i32
    }
}

fn main() {
    let nums = vec![3, 0, 1];
    let result = Solution::missing_number(nums);
    println!("Missing Number: {}", result);
}
