pub fn binary_search(nums: &Vec<i32>, target: i32) -> i32 {
  let mut low = 0;
  let mut high = nums.len() as i32 - 1;
  while low <= high {
    let mid = low + (high - low) / 2;
    if nums[mid as usize] == target {
      return mid;
    }

    if nums[mid as usize] < target {
      low = mid + 1;
    } else {
      high = mid - 1;
    }
  }

  -1
}
