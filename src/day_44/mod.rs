use std::collections::HashSet;

pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
  let word_set: HashSet<String> = std::collections::HashSet::from_iter(word_dict.iter().cloned());
  let mut dp = vec![false; s.len() + 1];
  dp[0] = true;

  for i in 1..=s.len() {
    for j in 0..i {
      if dp[j] && word_set.contains(&s[j..i].to_string()) {
        dp[i] = true;
        break;
      }
    }
  }

  dp[s.len()]
}

#[cfg(test)]
mod test {
  #[test]
  fn example_1() {
    let s = "leetcode".to_string();
    let word_dict = vec!["leet".to_string(), "code".to_string()];
    assert_eq!(super::word_break(s, word_dict), true);
  }

  #[test]
  fn example_2() {
    let s = "applepenapple".to_string();
    let word_dict = vec!["apple".to_string(), "pen".to_string()];
    assert_eq!(super::word_break(s, word_dict), true);
  }

  #[test]
  fn example_3() {
    let s = "catsandog".to_string();
    let word_dict = vec![
      "cats".to_string(),
      "dog".to_string(),
      "sand".to_string(),
      "and".to_string(),
      "cat".to_string(),
    ];
    assert_eq!(super::word_break(s, word_dict), false);
  }
}
