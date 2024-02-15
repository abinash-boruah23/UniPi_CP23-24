fn f(idx: usize, target: usize, arr: &[usize], dp: &mut Vec<Vec<Option<bool>>>) -> bool {
    if target == 0 {
        return true;
    }
    if idx == 0 {
        return arr[idx] == target;
    }
    if let Some(val) = dp[idx][target] {
        return val;
    }
    let nt = f(idx - 1, target, arr, dp);
    let mut t = false;
    if arr[idx] <= target {
        t = f(idx - 1, target - arr[idx], arr, dp);
    }
    dp[idx][target] = Some(nt || t);
    nt || t
}

fn equal_partition(n: usize, arr: &[usize]) -> i32 {
    let sum: usize = arr.iter().sum();
    if sum % 2 != 0 {
        return 0;
    }
    let target = sum / 2;
    let mut dp = vec![vec![None; target + 1]; n];
    f(n - 1, target, arr, &mut dp) as i32
}

fn main() {
    let arr = vec![1, 5, 11, 5];
    println!("{}", equal_partition(arr.len(), &arr)); // Output: 1
}
Partition Equal Subset Sum
