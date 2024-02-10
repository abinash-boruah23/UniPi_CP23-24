struct Solution;

impl Solution {
    fn find_first_element(nums: &Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() as i32 - 1;
        let mut start = -1;

        while l <= r {
            let m = (l + r) / 2;
            if nums[m as usize] == target {
                start = m;
                r = m - 1;
            } else if nums[m as usize] > target {
                r = m - 1;
            } else {
                l = m + 1;
            }
        }
        start
    }

    fn find_last_element(nums: &Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() as i32 - 1;
        let mut end = -1;

        while l <= r {
            let m = (l + r) / 2;
            if nums[m as usize] == target {
                end = m;
                l = m + 1;
            } else if nums[m as usize] > target {
                r = m - 1;
            } else {
                l = m + 1;
            }
        }
        end
    }

    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let start = Solution::find_first_element(&nums, target);
        let end = Solution::find_last_element(&nums, target);
        vec![start, end]
    }
}

fn main() {
    let nums = vec![5, 7, 7, 8, 8, 10];
    let target = 8;
    let result = Solution::search_range(nums, target);
    println!("Range of target element: {:?}", result);
}
