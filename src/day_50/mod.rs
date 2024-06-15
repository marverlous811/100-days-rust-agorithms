fn subset_sum(nums: &Vec<i32>, target: i32, idx: i32, dp: &mut Vec<Vec<i32>>) -> bool {
  if target == 0 {
    return true;
  }

  if idx == 0 {
    return nums[idx as usize] == target;
  }

  if dp[idx as usize][target as usize] != -1 {
    return dp[idx as usize][target as usize] == 1;
  }

  let not_taken = subset_sum(nums, target, idx - 1, dp);
  let mut taken = false;
  if nums[idx as usize] <= target {
    taken = subset_sum(nums, target - nums[idx as usize], idx - 1, dp);
  }

  dp[idx as usize][target as usize] = if not_taken || taken {
    1
  } else {
    0
  };

  dp[idx as usize][target as usize] == 1
}

pub fn can_partition(nums: Vec<i32>) -> bool {
  let sum: i32 = nums.iter().sum();
  if sum % 2 != 0 {
    false
  } else {
    let target = sum / 2;
    let mut dp = vec![vec![-1; (target + 1) as usize]; nums.len()];
    subset_sum(&nums, target, nums.len() as i32 - 1, &mut dp)
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn example_1() {
    let nums = vec![1, 5, 11, 5];
    let result = super::can_partition(nums);
    assert_eq!(result, true);
  }

  #[test]
  fn example_2() {
    let nums = vec![1, 2, 3, 5];
    let result = super::can_partition(nums);
    assert_eq!(result, false);
  }
}
