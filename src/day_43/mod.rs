pub fn edit_distance(word1: String, word2: String, m: i32, n: i32, cache: &mut Vec<Vec<i32>>) -> i32 {
  if m == 0 {
    return n;
  }
  if n == 0 {
    return m;
  }

  if cache[m as usize][n as usize] != -1 {
    return cache[m as usize][n as usize];
  }

  if word1.chars().nth(m as usize - 1).unwrap() == word2.chars().nth(n as usize - 1).unwrap() {
    return edit_distance(word1, word2, m - 1, n - 1, cache);
  }

  let result = 1
    + std::cmp::min(
      std::cmp::min(
        edit_distance(word1.clone(), word2.clone(), m, n - 1, cache), // Insert
        edit_distance(word1.clone(), word2.clone(), m - 1, n, cache), // Remove
      ),
      edit_distance(word1.clone(), word2.clone(), m - 1, n - 1, cache), //Replace
    );

  cache[m as usize][n as usize] = result;
  result
}

pub fn min_distance(word1: String, word2: String) -> i32 {
  let mut cache = vec![vec![-1; word2.len() + 1]; word1.len() + 1];
  edit_distance(
    word1.clone(),
    word2.clone(),
    word1.len() as i32,
    word2.len() as i32,
    &mut cache,
  )
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_min_distance() {
    assert_eq!(min_distance("horse".to_string(), "ros".to_string()), 3);
  }

  #[test]
  fn test_min_distance_2() {
    assert_eq!(min_distance("intention".to_string(), "execution".to_string()), 5);
  }

  #[test]
  fn test_min_distance_3() {
    assert_eq!(
      min_distance(
        "dinitrophenylhydrazine".to_string(),
        "benzalphenylhydrazone".to_string()
      ),
      7
    );
  }
}
