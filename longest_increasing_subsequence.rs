pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut v = Vec::new();
    v.push(nums[0]);
    for &num in &nums[1..] {
        if *v.last().unwrap() < num {
            v.push(num);
        } else {
            let mut l = 0;
            let mut r = v.len() - 1;
            while l <= r {
                let m = (l + r) / 2;
                if v[m] < num {
                    l = m + 1;
                } else {
                    if m > 0 {
                        r = m - 1;
                    } else {
                        break; // Exiting the loop if m is already 0 to avoid underflow
                    }
                }
            }
            v[l] = num;
        }
    }
    v.len() as i32
}

fn main() {
    let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
    println!("{}", length_of_lis(nums)); // Output: 4
}
