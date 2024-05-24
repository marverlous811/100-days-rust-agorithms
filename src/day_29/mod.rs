use crate::common::binary_search;

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
  let idx = binary_search(&nums, target);

  if idx == -1 {
    vec![-1, -1]
  } else {
    let mut start = idx;
    let mut end = idx;

    while start > 0 && nums[(start - 1) as usize] == target {
      start -= 1;
    }

    while end < nums.len() as i32 - 1 && nums[(end + 1) as usize] == target {
      end += 1;
    }

    vec![start, end]
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_search_range() {
    assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
    assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 6), vec![-1, -1]);
    assert_eq!(search_range(vec![], 0), vec![-1, -1]);
  }
}
