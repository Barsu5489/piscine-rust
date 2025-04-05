pub fn edit_distance(source: &str, target: &str) -> usize {
    let m = source.len(); // Length of source string
    let n = target.len(); // Length of target string
    let s = source.as_bytes(); // Convert source to bytes for indexing
    let t = target.as_bytes(); // Convert target to bytes for indexing

    // Create a 2D table (m+1 rows, n+1 columns) to store edit distances
    let mut dp = vec![vec![0; n + 1]; m + 1];

    // Fill first row and first column with base cases
    for i in 0..=m {
        dp[i][0] = i; // If target is empty, we need i deletions
    }

    for j in 0..=n {
        dp[0][j] = j; // If source is empty, we need j insertions
    }

    // Fill the DP table
    for i in 1..=m {
        for j in 1..=n {
            if s[i - 1] == t[j - 1] {
                dp[i][j] = dp[i - 1][j - 1]; // Characters match, no change
            } else {
                dp[i][j] = 1 + std::cmp::min(
                    dp[i - 1][j - 1], // Substitution
                    std::cmp::min(dp[i][j - 1], dp[i - 1][j]), // Insertion or Deletion
                );
            }
        }
    }

    dp[m][n] // Return the final result from the bottom-right corner
}