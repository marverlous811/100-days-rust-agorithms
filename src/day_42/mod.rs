fn min_cost_path(grid: Vec<Vec<i32>>, m: i32, n: i32, memo: &mut Vec<Vec<i32>>) -> i32 {
  if m < 0 || n < 0 {
    i32::MAX
  } else if m == 0 && n == 0 {
    grid[m as usize][n as usize]
  } else {
    if memo[m as usize][n as usize] != -1 {
      return memo[m as usize][n as usize];
    }

    memo[m as usize][n as usize] = grid[m as usize][n as usize]
      + std::cmp::min(
        min_cost_path(grid.clone(), m - 1, n, memo),
        min_cost_path(grid.clone(), m, n - 1, memo),
      );

    memo[m as usize][n as usize]
  }
}

pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
  let y = grid.len();
  let x = grid[0].len();
  let mut memo = vec![vec![-1; x]; y];
  min_cost_path(grid, y as i32 - 1, x as i32 - 1, &mut memo)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_min_path_sum() {
    assert_eq!(min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]), 7);
  }

  #[test]
  fn test_min_path_sum_2() {
    assert_eq!(min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]), 12);
  }

  #[test]
  fn test_min_path_sum_3() {
    assert_eq!(
      min_path_sum(vec![
        vec![7, 1, 3, 5, 8, 9, 9, 2, 1, 9, 0, 8, 3, 1, 6, 6, 9, 5],
        vec![9, 5, 9, 4, 0, 4, 8, 8, 9, 5, 7, 3, 6, 6, 6, 9, 1, 6],
        vec![8, 2, 9, 1, 3, 1, 9, 7, 2, 5, 3, 1, 2, 4, 8, 2, 8, 8],
        vec![6, 7, 9, 8, 4, 8, 3, 0, 4, 0, 9, 6, 6, 0, 0, 5, 1, 4],
        vec![7, 1, 3, 1, 8, 8, 3, 1, 2, 1, 5, 0, 2, 1, 9, 1, 1, 4],
        vec![9, 5, 4, 3, 5, 6, 1, 3, 6, 4, 9, 7, 0, 8, 0, 3, 9, 9],
        vec![1, 4, 2, 5, 8, 7, 7, 0, 0, 7, 1, 2, 1, 2, 7, 7, 7, 4],
        vec![3, 9, 7, 9, 5, 8, 9, 5, 6, 9, 8, 8, 0, 1, 4, 2, 8, 2],
        vec![1, 5, 2, 2, 2, 5, 6, 3, 9, 3, 1, 7, 9, 6, 8, 6, 8, 3],
        vec![5, 7, 8, 3, 8, 8, 3, 9, 9, 8, 1, 9, 2, 5, 4, 7, 7, 7],
        vec![2, 3, 2, 4, 8, 5, 1, 7, 2, 9, 5, 2, 4, 2, 9, 2, 8, 7],
        vec![0, 1, 6, 1, 1, 0, 0, 6, 5, 4, 3, 4, 3, 7, 9, 6, 1, 9]
      ]),
      85
    );
  }
}
