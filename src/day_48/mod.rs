pub fn recursive(nums: &Vec<i32>, end: usize, max: &mut i32) -> i32 {
  if end == 1 {
    return 1;
  }
  let mut res = 1;
  let mut max_ending_here = 1;

  for i in 1..end {
    res = recursive(nums, i, max);
    if nums[i - 1] < nums[end - 1] && res + 1 > max_ending_here {
      max_ending_here = res + 1;
    }
  }

  if *max < max_ending_here {
    *max = max_ending_here;
  }

  max_ending_here
}

pub fn memoization(nums: &Vec<i32>, idx: i32, prev_idx: i32, dp: &mut Vec<Vec<i32>>) -> i32 {
  if idx == nums.len() as i32 {
    return 0;
  }

  if dp[idx as usize][(prev_idx + 1) as usize] != -1 {
    return dp[idx as usize][prev_idx as usize + 1];
  }

  let not_take = 0 + memoization(nums, idx + 1, prev_idx, dp);
  let mut take = i32::MIN;
  if prev_idx < 0 || nums[idx as usize] > nums[prev_idx as usize] {
    take = 1 + memoization(nums, idx + 1, idx, dp);
  }

  dp[idx as usize][(prev_idx + 1) as usize] = std::cmp::max(not_take, take);

  dp[idx as usize][(prev_idx + 1) as usize]
}

pub fn length_of_lis(nums: Vec<i32>) -> i32 {
  let mut dp = vec![vec![-1; nums.len() + 1]; nums.len() + 1];
  memoization(&nums, 0, -1, &mut dp)
}

#[cfg(test)]
mod test {
  use crate::helper::read_i32_input_from_file;

  #[test]
  fn example_1() {
    let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
    assert_eq!(super::length_of_lis(nums), 4);
  }

  #[test]
  fn example_2() {
    let nums = vec![0, 1, 0, 3, 2, 3];
    assert_eq!(super::length_of_lis(nums), 4);
  }

  #[test]
  fn example_3() {
    let nums = vec![7, 7, 7, 7, 7, 7, 7];
    assert_eq!(super::length_of_lis(nums), 1);
  }

  #[test]
  fn example_4() {
    let input = read_i32_input_from_file("src/day_48/sample.txt");
    assert_eq!(super::length_of_lis(input), 2500);
  }
}
