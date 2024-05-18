pub fn generator(candidates: Vec<i32>, target: i32, start: i32, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
  if start >= candidates.len() as i32 {
    return;
  }

  if target < 0 {
    return;
  }

  if target == 0 {
    res.push(path.to_vec());
    return;
  }

  for i in start..candidates.len() as i32 {
    path.push(candidates[i as usize]);
    generator(candidates.to_vec(), target - candidates[i as usize], i, path, res);
    path.pop();
  }
}

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
  let mut retval: Vec<Vec<i32>> = vec![];
  let mut path: Vec<i32> = vec![];
  generator(candidates, target, 0, &mut path, &mut retval);

  retval
}

#[cfg(test)]
mod test {
  #[test]
  fn example_1() {
    assert_eq!(
      super::combination_sum(vec![2, 3, 6, 7], 7),
      vec![vec![2, 2, 3], vec![7]]
    );
  }

  #[test]
  fn example_2() {
    assert_eq!(
      super::combination_sum(vec![2, 3, 5], 8),
      vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
    );
  }

  #[test]
  fn example_3() {
    assert_eq!(super::combination_sum(vec![2], 1), vec![] as Vec<Vec<i32>>);
  }
}
