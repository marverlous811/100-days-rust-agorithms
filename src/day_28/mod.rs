pub fn binary_search(nums: &Vec<i32>, target: i32, low: i32, high: i32) -> i32 {
  // println!("low: {}, high: {}", low, high);
  if low > high {
    return -1;
  }
  let mid = low + (high - low) / 2;
  // println!("low: {}, high: {}, mid: {}", low, high, mid);
  if target == nums[mid as usize] {
    return mid as i32;
  }

  if target < nums[mid as usize] {
    return binary_search(nums, target, low, mid - 1);
  }

  return binary_search(nums, target, mid + 1, high);
}

pub fn find_pivot(nums: &Vec<i32>, low: i32, high: i32) -> i32 {
  if low > high {
    return -1;
  }
  if low == high {
    return low as i32;
  }

  let mid = low + (high - low) / 2;
  if mid < high && nums[mid as usize] > nums[(mid + 1) as usize] {
    return mid as i32;
  }

  if mid > low && nums[mid as usize] < nums[(mid - 1) as usize] {
    return (mid - 1) as i32;
  }

  if nums[low as usize] >= nums[mid as usize] {
    return find_pivot(nums, low, mid - 1);
  }

  return find_pivot(nums, mid + 1, high);
}

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
  let last_idx = nums.len() - 1;
  let pivot = find_pivot(&nums, 0, last_idx as i32);
  match pivot {
    -1 => binary_search(&nums, target, 0, last_idx as i32),
    _ => {
      if nums[pivot as usize] == target {
        return pivot;
      }

      if nums[0] <= target {
        return binary_search(&nums, target, 0, pivot - 1);
      }

      return binary_search(&nums, target, pivot + 1, last_idx as i32);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_search() {
    assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
  }

  #[test]
  fn test_search_2() {
    assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
  }

  #[test]
  fn test_search_3() {
    assert_eq!(search(vec![1], 0), -1);
  }

  #[test]
  fn test_search_4() {
    assert_eq!(search(vec![1], 2), -1);
  }

  #[test]
  fn test_search_5() {
    assert_eq!(search(vec![1, 3], 0), -1);
  }
}
