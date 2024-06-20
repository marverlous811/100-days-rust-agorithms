fn jump_through(nums: &Vec<i32>, curr: usize, dp: &mut Vec<i32>) -> i32 {
  if curr >= nums.len() - 1 {
    return 0;
  }

  if dp[curr] != -1 {
    return dp[curr];
  }

  let mut min = i32::MAX;
  let max_move = nums[curr] as usize;
  for i in 1..=max_move {
    let next = curr + i;
    if next >= nums.len() {
      break;
    }
    let next_res = jump_through(nums, next, dp);
    min = min.min(if next_res == i32::MAX {
      i32::MAX
    } else {
      next_res + 1
    });
  }

  dp[curr] = min;
  dp[curr]
}

pub fn jump(nums: Vec<i32>) -> i32 {
  let mut dp = vec![-1; nums.len() + 1];
  jump_through(&nums, 0, &mut dp)
}

#[cfg(test)]
mod test {
  #[test]
  fn example_1() {
    let nums = vec![2, 3, 1, 1, 4];
    assert_eq!(super::jump(nums), 2);
  }

  #[test]
  fn example_2() {
    let nums = vec![2, 3, 0, 1, 4];
    assert_eq!(super::jump(nums), 2);
  }
}
