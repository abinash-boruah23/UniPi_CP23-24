use std::cmp;

struct Solution;

impl Solution {
    fn f(coins: &Vec<i32>, amount: i32, n: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
        if n == 0 {
            if amount % coins[n] == 0 {
                return amount / coins[n];
            } else {
                return i32::MAX;
            }
        }

        if dp[n][amount as usize] != -1 {
            return dp[n][amount as usize];
        }

        let not_take = Solution::f(coins, amount, n - 1, dp);
        let mut take = i32::MAX;

        if amount >= coins[n] {
            take = 1 + Solution::f(coins, amount - coins[n], n, dp);
        }

        dp[n][amount as usize] = cmp::min(not_take, take);
        dp[n][amount as usize]
    }

    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let n = coins.len();
        let mut dp = vec![vec![-1; (amount + 1) as usize]; n];
        let ans = Solution::f(&coins, amount, n - 1, &mut dp);

        if ans >= i32::MAX {
            return -1;
        }

        ans
    }
}

fn main() {
    let coins = vec![1, 2, 5];
    let amount = 11;
    let result = Solution::coin_change(coins, amount);
    println!("Minimum number of coins: {}", result);
}
