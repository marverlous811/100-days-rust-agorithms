pub fn number_unique_paths(m: i32, n: i32, cache: &mut Vec<Vec<i32>>) -> i32 {
  if m == 1 || n == 1 {
    cache[m as usize][n as usize] = 1;
    return 1;
  }

  if cache[m as usize][n as usize] == 0 {
    cache[m as usize][n as usize] = number_unique_paths(m - 1, n, cache) + number_unique_paths(m, n - 1, cache);
  }

  cache[m as usize][n as usize]
}

pub fn unique_paths(m: i32, n: i32) -> i32 {
  let mut cache = vec![vec![0 as i32; n as usize + 1]; m as usize + 1];

  number_unique_paths(m, n, &mut cache)
}

#[cfg(test)]
mod test {
  #[test]
  fn example_1() {
    assert_eq!(super::unique_paths(3, 7), 28);
  }

  #[test]
  fn example_2() {
    assert_eq!(super::unique_paths(3, 2), 3);
  }

  #[test]
  fn example_3() {
    assert_eq!(super::unique_paths(10, 10), 48620);
  }

  #[test]
  fn example_4() {
    assert_eq!(super::unique_paths(23, 12), 193536720);
  }

  #[test]
  fn example_5() {
    assert_eq!(super::unique_paths(51, 9), 1916797311);
  }
}
