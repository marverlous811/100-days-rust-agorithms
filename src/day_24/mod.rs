fn backtrack(nums: &mut Vec<i32>, left: usize, right: usize, result: &mut Vec<Vec<i32>>) {
  if left == right {
    result.push(nums.clone());
  } else {
    for i in left..right {
      nums.swap(left, i);
      backtrack(&mut nums.clone(), left + 1, right, result);
      nums.swap(left, i);
    }
  }
}

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
  let mut retval = vec![];

  backtrack(&mut nums.clone(), 0, nums.len(), &mut retval);

  retval
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_permute_1() {
    let input = vec![1, 2, 3];
    let expected = vec![
      vec![1, 2, 3],
      vec![1, 3, 2],
      vec![2, 1, 3],
      vec![2, 3, 1],
      vec![3, 2, 1],
      vec![3, 1, 2],
    ];
    assert_eq!(permute(input), expected);
  }

  #[test]
  fn test_permute_2() {
    let input = vec![0, 1];
    let expected = vec![vec![0, 1], vec![1, 0]];
    assert_eq!(permute(input), expected);
  }

  #[test]
  fn test_permute_3() {
    let input = vec![1];
    let expected = vec![vec![1]];
    assert_eq!(permute(input), expected);
  }
}
