
// Edit distance function (Levenshtein distance)
pub fn edit_distance(s1: &str, s2: &str) -> usize {
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();
    let len1 = s1_chars.len();
    let len2 = s2_chars.len();
    
    let mut dp = vec![vec![0; len2 + 1]; len1 + 1];
    
    // Initialize first row and column
    for i in 0..=len1 {
        dp[i][0] = i;
    }
    for j in 0..=len2 {
        dp[0][j] = j;
    }
    
    // Fill the DP table
    for i in 1..=len1 {
        for j in 1..=len2 {
            if s1_chars[i-1] == s2_chars[j-1] {
                dp[i][j] = dp[i-1][j-1];
            } else {
                dp[i][j] = 1 + dp[i-1][j-1].min(dp[i-1][j]).min(dp[i][j-1]);
            }
        }
    }
    
    dp[len1][len2]
}