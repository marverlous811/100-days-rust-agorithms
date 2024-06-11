pub fn max_product(nums: Vec<i32>) -> i32 {
  let mut max_ending_here = nums[0] as i64;
  let mut min_ending_here = nums[0] as i64;
  let mut result = nums[0] as i64;

  for i in 1..nums.len() {
    let product_max_ending = max_ending_here * nums[i] as i64;
    let product_min_ending = std::cmp::max(min_ending_here * nums[i] as i64, i32::MIN as i64);

    min_ending_here = std::cmp::min(nums[i] as i64, std::cmp::min(product_max_ending, product_min_ending));
    max_ending_here = std::cmp::max(nums[i] as i64, std::cmp::max(product_max_ending, product_min_ending));
    result = std::cmp::max(max_ending_here, result);
  }

  result as i32
}

#[cfg(test)]
mod test {
  use crate::helper::read_i32_input_from_file;

  use super::*;

  #[test]
  fn example_1() {
    assert_eq!(max_product(vec![2, 3, -2, 4]), 6);
  }

  #[test]
  fn example_2() {
    assert_eq!(max_product(vec![-2, 0, -1]), 0);
  }

  #[test]
  fn example_3() {
    assert_eq!(max_product(vec![-2]), -2);
  }

  #[test]
  fn example_4() {
    assert_eq!(max_product(vec![0, 2]), 2);
  }

  #[test]
  fn example_5() {
    let input = read_i32_input_from_file("src/day_45/sample.txt");
    assert_eq!(max_product(input), 1492992000);
  }

  #[test]
  fn example_6() {
    assert_eq!(
      max_product(vec![
        0, 10, 10, 10, 10, 10, 10, 10, 10, 10, -10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 0
      ]),
      1000000000
    )
  }
}
