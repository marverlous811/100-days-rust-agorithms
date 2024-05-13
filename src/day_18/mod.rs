pub fn move_zeroes(nums: &mut Vec<i32>) {
  let mut i = 0;
  let swap = |nums: &mut Vec<i32>, i: usize, j: usize| {
    let tmp = nums[i];
    nums[i] = nums[j];
    nums[j] = tmp;
  };
  while i < nums.len() - 1 {
    if nums[i] == 0 {
      let mut j = i + 1;
      while j < nums.len() && nums[j] == 0 {
        j += 1;
      }
      if j < nums.len() {
        swap(nums, i, j);
      } else {
        break;
      }
    } else {
      i += 1;
    }
  }
}

pub fn move_zeroes_optz(nums: &mut Vec<i32>) {
  let mut j = 0;
  for i in 0..nums.len() {
    if nums[i] == 0 {
      continue;
    }
    nums.swap(i, j);
    j += 1;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_move_zeroes_1() {
    let mut nums = vec![0, 1, 0, 3, 12];
    move_zeroes_optz(&mut nums);
    assert_eq!(nums, vec![1, 3, 12, 0, 0]);
  }

  #[test]
  fn test_move_zeroes_2() {
    let mut nums = vec![0];
    move_zeroes(&mut nums);
    assert_eq!(nums, vec![0]);
  }
}
