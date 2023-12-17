use std::collections::VecDeque;

fn max_sliding_window(nums: Vec<i32>, k: usize) -> Vec<i32> {
    let mut dq: VecDeque<usize> = VecDeque::new();
    let mut result: Vec<i32> = Vec::new();

    for i in 0..nums.len() {
        while !dq.is_empty() && nums[*dq.back().unwrap()] < nums[i] {
            dq.pop_back();
        }
        dq.push_back(i);

        if i >= k - 1 {
            if let Some(front) = dq.front().cloned() {
                if i.checked_sub(k) == Some(front) {
                    dq.pop_front();
                }
            }
            result.push(nums[*dq.front().unwrap()]);
        }
    }

    result
}

fn main() {
    let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3;
    let result = max_sliding_window(nums, k);
    println!("{:?}", result);
}
