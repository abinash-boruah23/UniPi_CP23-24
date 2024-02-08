struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut max_l = height[l];
        let mut max_r = height[r];
        let mut res = 0;

        while l < r {
            if height[l] <= height[r] {
                if max_l < height[l] {
                    max_l = height[l];
                } else {
                    res += max_l - height[l];
                }
                l += 1;
            } else {
                if max_r < height[r] {
                    max_r = height[r];
                } else {
                    res += max_r - height[r];
                }
                r -= 1;
            }
        }
        res
    }
}

fn main() {
    let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    let result = Solution::trap(height);
    println!("Amount of water trapped: {}", result);
}
