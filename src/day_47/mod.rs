pub fn num_squares_down(n: i32, cache: &mut Vec<i32>) -> i32 {
  if n == 0 {
    return 0;
  }

  if n == 1 {
    return 1;
  }

  let max = (n as f32).sqrt().floor() as i32;
  let mut retval = i32::MAX;
  for i in 1..=max {
    let square = if cache[i as usize - 1] == -1 {
      let square: i32 = i * i;
      cache[i as usize - 1] = square;
      square
    } else {
      cache[i as usize - 1]
    };
    retval = retval.min(1 + num_squares_down(n - square, cache));
  }

  retval
}

pub fn num_squares(n: i32) -> i32 {
  let mut dp = vec![n; n as usize + 1];
  dp[0] = 0;
  dp[1] = 1;

  for i in 2..=n {
    let mut j = 1;
    while j * j <= i {
      dp[i as usize] = std::cmp::min(dp[i as usize], dp[i as usize - (j * j) as usize] + 1);
      j += 1;
    }
  }

  dp[n as usize]
}

#[cfg(test)]
mod test {
  #[test]
  fn example_1() {
    assert_eq!(super::num_squares(12), 3);
  }

  #[test]
  fn example_2() {
    assert_eq!(super::num_squares(13), 2);
  }

  #[test]
  fn example_3() {
    assert_eq!(super::num_squares(2), 2);
  }

  #[test]
  fn example_4() {
    assert_eq!(super::num_squares(4), 1);
  }

  #[test]
  fn example_5() {
    assert_eq!(super::num_squares(54), 3);
  }
}
