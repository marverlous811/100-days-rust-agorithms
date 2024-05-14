use std::collections::HashMap;

pub fn single_number(nums: Vec<i32>) -> i32 {
  let mut map = HashMap::<i32, i32>::new();
  for i in 0..nums.len() {
    let count = map.entry(nums[i]).or_insert(0);
    *count += 1;
  }

  for (key, value) in map.iter() {
    if *value == 1 {
      return *key;
    }
  }
  0
}

pub fn single_number_optz(nums: Vec<i32>) -> i32 {
  let mut result = 0;
  for i in 0..nums.len() {
    println!("{}", nums[i]);
    result ^= nums[i];
    println!("{}", result);
    println!("=========================");
  }

  result
}

#[cfg(test)]
mod test {
  #[test]
  fn single_number_test_1() {
    assert_eq!(super::single_number_optz(vec![2, 2, 1]), 1);
  }

  #[test]
  fn single_number_test_2() {
    assert_eq!(super::single_number(vec![4, 1, 2, 1, 2]), 4);
  }

  #[test]
  fn single_number_test_3() {
    assert_eq!(super::single_number(vec![1]), 1);
  }
}
