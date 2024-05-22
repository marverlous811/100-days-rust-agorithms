pub fn is_str_palindrome(s: &str) -> bool {
  let mut left = 0;
  let mut right = s.len() - 1;

  while left < right {
    if s.chars().nth(left).unwrap() != s.chars().nth(right).unwrap() {
      return false;
    }

    left += 1;
    right -= 1;
  }

  true
}

pub fn backtrack_partition(s: &str, start: usize, path: &mut Vec<String>, res: &mut Vec<Vec<String>>) {
  if start == s.len() {
    res.push(path.clone());
    return;
  }

  let mut cur = String::new();
  for i in start..s.len() {
    cur.push(s.chars().nth(i).unwrap());

    if is_str_palindrome(cur.as_str()) {
      path.push(cur.clone());
      backtrack_partition(s, i + 1, path, res);
      path.pop();
    }
  }
}

pub fn partition(s: String) -> Vec<Vec<String>> {
  let mut res = vec![];
  let mut path = vec![];

  backtrack_partition(&s, 0, &mut path, &mut res);

  res
}

#[cfg(test)]
mod test {
  #[test]
  fn test_partition_1() {
    assert_eq!(
      super::partition("aab".to_string()),
      vec![vec!["a", "a", "b"], vec!["aa", "b"],]
    );
  }

  #[test]
  fn test_partition_2() {
    assert_eq!(super::partition("a".to_string()), vec![vec!["a"],]);
  }
}
