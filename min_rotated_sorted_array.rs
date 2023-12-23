struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let n = nums.len() - 1;

        if nums[0] <= nums[n] {
            return nums[0];
        }

        let mut l = 0;
        let mut r = n;

        while l < r {
            let mid = (l + r) / 2;

            if nums[mid] > nums[0] {
                l = mid + 1;
            } else {
                r = mid;
            }
        }

        nums[l]
    }
}

fn main() {
    let nums = vec![3,4,5,1,2];
    let result = Solution::find_min(nums);
    println!("Minimum Element: {}", result);
}
