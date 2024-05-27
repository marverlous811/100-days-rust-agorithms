fn find_pivot(nums: &Vec<i32>, left: i32, right: i32) -> i32 {
  if left > right {
    return -1;
  }

  if left == right {
    return left;
  }

  let mid = left + (right - left) / 2;
  println!("left: {}, mid: {}, right: {}", left, mid, right);
  if mid < right && nums[mid as usize] > nums[mid as usize + 1] {
    println!("mid > right");
    return mid;
  }

  if mid > left && nums[mid as usize] < nums[mid as usize - 1] {
    println!("mid < left");
    return mid - 1;
  }

  if nums[left as usize] >= nums[mid as usize] {
    println!("find in left side");
    return find_pivot(nums, left, mid - 1);
  }

  println!("find in right side");
  return find_pivot(nums, mid + 1, right);
}

pub fn find_min(nums: Vec<i32>) -> i32 {
  let pivot = find_pivot(&nums, 0, nums.len() as i32 - 1);
  println!("pivot: {}", pivot);
  if pivot == nums.len() as i32 - 1 {
    nums[0]
  } else {
    nums[(pivot + 1) as usize]
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_find_min() {
    assert_eq!(find_min(vec![3, 4, 5, 1, 2]), 1);
  }

  #[test]
  fn test_find_min_2() {
    assert_eq!(find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
  }

  #[test]
  fn test_find_min_3() {
    assert_eq!(find_min(vec![11, 13, 15, 17]), 11);
  }

  #[test]
  fn test_find_min_4() {
    assert_eq!(find_min(vec![1]), 1);
  }

  #[test]
  fn test_find_min_5() {
    assert_eq!(find_min(vec![1, 2, 3]), 1);
  }
}
