pub fn edit_distance(source: &str, target: &str) -> usize {
    let source: Vec<char> = source.chars().collect();
    let target: Vec<char> = target.chars().collect();
    let m = source.len();
    let n = target.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }
    for i in 1..=m {
        for j in 1..=n {
            if source[i - 1] == target[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j]
                    .min(dp[i][j - 1])
                    .min(dp[i - 1][j - 1]);
            }
        }
    }
    dp[m][n]
}