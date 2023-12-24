use std::cmp;

fn knapsack_memo(weight: &Vec<i32>, value: &Vec<i32>, n: usize, max_weight: i32, dp: &mut Vec<Vec<i32>>) -> i32 {
    if n == 0 {
        if max_weight >= weight[0] {
            return value[0];
        }
        return 0;
    }

    if dp[n][max_weight as usize] != -1 {
        return dp[n][max_weight as usize];
    }

    let not_take = knapsack_memo(weight, value, n - 1, max_weight, dp);
    let mut take = i32::MIN;
    if max_weight >= weight[n] {
        take = value[n] + knapsack_memo(weight, value, n - 1, max_weight - weight[n], dp);
    }

    dp[n][max_weight as usize] = cmp::max(take, not_take);
    dp[n][max_weight as usize]
}

fn main() {
    let weight = vec![1, 2, 4, 5];
    let value = vec![5, 4, 8, 6];
    let n = 4;
    let max_weight = 5;
    let mut dp = vec![vec![-1; (max_weight + 1) as usize]; n];

    println!("{}", knapsack_memo(&weight, &value, n - 1, max_weight, &mut dp));
}
