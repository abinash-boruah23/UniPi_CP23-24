struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let idx1 = text1.len();
        let idx2 = text2.len();
        let mut dp = vec![vec![0; idx2 + 1]; idx1 + 1];

        for i in 0..=idx1 {
            for j in 0..=idx2 {
                if i == 0 || j == 0 {
                    dp[i][j] = 0;
                } else {
                    if text1.chars().nth(i - 1).unwrap() == text2.chars().nth(j - 1).unwrap() {
                        dp[i][j] = 1 + dp[i - 1][j - 1];
                    } else {
                        let move_first = dp[i - 1][j];
                        let move_second = dp[i][j - 1];
                        dp[i][j] = move_first.max(move_second);
                    }
                }
            }
        }

        dp[idx1][idx2]
    }
}

fn main() {
    let text1 = "abcde".to_string();
    let text2 = "ace".to_string();
    let result = Solution::longest_common_subsequence(text1, text2);
    println!("Length of Longest Common Subsequence: {}", result);
}
