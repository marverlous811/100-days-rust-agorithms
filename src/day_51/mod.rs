fn lcs(t1: &String, t2: &String, i: usize, j: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
  if i == 0 || j == 0 {
    return 0;
  }

  if t1.chars().nth(i - 1).unwrap() == t2.chars().nth(j - 1).unwrap() {
    dp[i][j] = 1 + lcs(t1, t2, i - 1, j - 1, dp);
    return dp[i][j];
  }

  if dp[i][j] != -1 {
    return dp[i][j];
  }

  dp[i][j] = std::cmp::max(lcs(t1, t2, i, j - 1, dp), lcs(t1, t2, i - 1, j, dp));
  dp[i][j]
}

pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
  let m = text1.len();
  let n = text2.len();
  let mut dp = vec![vec![-1; n + 1]; m + 1];

  for i in 0..=m {
    for j in 0..=n {
      if i == 0 || j == 0 {
        dp[i][j] = 0
      } else if text1.chars().nth(i - 1).unwrap() == text2.chars().nth(j - 1).unwrap() {
        dp[i][j] = 1 + dp[i - 1][j - 1];
      } else {
        dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);
      }
    }
  }

  dp[m][n]
}

#[cfg(test)]
mod test {
  #[test]
  fn example_1() {
    let text1 = "abcde".to_string();
    let text2 = "ace".to_string();
    assert_eq!(super::longest_common_subsequence(text1, text2), 3);
  }

  #[test]
  fn example_2() {
    let text1 = "abc".to_string();
    let text2 = "abc".to_string();
    assert_eq!(super::longest_common_subsequence(text1, text2), 3);
  }

  #[test]
  fn example_3() {
    let text1 = "abc".to_string();
    let text2 = "def".to_string();
    assert_eq!(super::longest_common_subsequence(text1, text2), 0);
  }

  #[test]
  fn example_4() {
    let text1 = "ccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc".to_string();
    let text2 = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_string();
    assert_eq!(super::longest_common_subsequence(text1, text2), 0);
  }
}
