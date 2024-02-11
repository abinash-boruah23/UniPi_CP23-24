struct Solution;

impl Solution {
    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        let mut v = vec![0; 52];
        
        for i in &ranges {
            v[i[0] as usize] += 1;
            v[(i[1] + 1) as usize] -= 1;
        }
        
        let mut sum = 0;
        for i in 1..=right as usize {
            sum += v[i];
            if sum == 0 && i as i32 >= left {
                return false;
            }
        }
        true
    }
}

fn main() {
    let ranges = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let left = 2;
    let right = 5;
    println!("{}", Solution::is_covered(ranges, left, right)); // Output: true
}
