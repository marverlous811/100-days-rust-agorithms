pub fn backtrack(nums: Vec<i32>, size: usize, start: usize, tmp: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
  if tmp.len() == size {
    // println!("{:?}", tmp);
    res.push(tmp.clone());
  }
  if size == nums.len() {
    return;
  }

  for i in start..nums.len() {
    tmp.push(nums[i]);
    backtrack(nums.clone(), size + 1, i + 1, tmp, res);
    tmp.pop();
  }
}

pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
  let mut res = vec![];
  let mut tmp = vec![];

  backtrack(nums, 0, 0, &mut tmp, &mut res);

  res
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_subsets() {
    assert_eq!(
      subsets(vec![1, 2, 3]),
      vec![
        vec![],
        vec![1],
        vec![1, 2],
        vec![1, 2, 3],
        vec![1, 3],
        vec![2],
        vec![2, 3],
        vec![3]
      ]
    );
  }

  #[test]
  fn test_subsets_2() {
    assert_eq!(subsets(vec![0]), vec![vec![], vec![0]]);
  }
}
