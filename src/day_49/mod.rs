pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
  let mut dp = vec![i32::MAX; amount as usize + 1];
  dp[0] = 0;
  for a in 1..amount + 1 {
    for c in coins.iter() {
      if a - c >= 0 {
        let x = if dp[(a - c) as usize] == i32::MAX {
          i32::MAX
        } else {
          dp[(a - c) as usize] + 1
        };
        dp[a as usize] = std::cmp::min(dp[a as usize], x);
      }
    }
  }

  if dp[amount as usize] != i32::MAX && dp[amount as usize] != amount + 1 {
    dp[amount as usize]
  } else {
    -1
  }
}

#[cfg(test)]
mod test {
  #[test]
  fn example_1() {
    let coins = vec![1, 2, 5];
    let amount = 11;
    assert_eq!(super::coin_change(coins, amount), 3);
  }

  #[test]
  fn example_2() {
    let coins = vec![2];
    let amount = 3;
    assert_eq!(super::coin_change(coins, amount), -1);
  }

  #[test]
  fn example_3() {
    let coins = vec![1];
    let amount = 0;
    assert_eq!(super::coin_change(coins, amount), 0);
  }

  #[test]
  fn example_4() {
    let coins = vec![1, 2, 5];
    let amount = 100;
    assert_eq!(super::coin_change(coins, amount), 20);
  }
}
