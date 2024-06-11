pub fn rob_through(nums: &Vec<i32>, n: i32, dp: &mut Vec<i32>) -> i32 {
  if n < 0 {
    return 0;
  }

  if n == 0 {
    return nums[0];
  }

  if dp[n as usize] != -1 {
    return dp[n as usize];
  }

  let pick = nums[n as usize] + rob_through(nums, n - 2, dp);
  let not_pick = rob_through(nums, n - 1, dp);

  dp[n as usize] = std::cmp::max(pick, not_pick);
  dp[n as usize]
}

pub fn rob(nums: Vec<i32>) -> i32 {
  let mut dp = vec![-1; nums.len()];
  rob_through(&nums, nums.len() as i32 - 1, &mut dp)
}

#[cfg(test)]
mod test {
  #[test]
  fn example_1() {
    assert_eq!(super::rob(vec![1, 2, 3, 1]), 4);
  }

  #[test]
  fn example_2() {
    assert_eq!(super::rob(vec![2, 7, 9, 3, 1]), 12);
  }

  #[test]
  fn example_3() {
    assert_eq!(super::rob(vec![2, 1, 1, 2]), 4);
  }

  #[test]
  fn example_4() {
    assert_eq!(super::rob(vec![1, 2]), 2);
  }

  #[test]
  fn example_5() {
    assert_eq!(
      super::rob(vec![
        114, 117, 207, 117, 235, 82, 90, 67, 143, 146, 53, 108, 200, 91, 80, 223, 58, 170, 110, 236, 81, 90, 222, 160,
        165, 195, 187, 199, 114, 235, 197, 187, 69, 129, 64, 214, 228, 78, 188, 67, 205, 94, 205, 169, 241, 202, 144,
        240
      ]),
      4173
    );
  }
}
