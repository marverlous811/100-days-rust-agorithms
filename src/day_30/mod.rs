use crate::common::binary_search;

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
  for row in 0..matrix.len() {
    let row_data = &matrix[row];
    if row_data[0] <= target && row_data[row_data.len() - 1] >= target {
      return binary_search(row_data, target) != -1;
    }
  }

  false
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_search_matrix() {
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    assert_eq!(search_matrix(matrix, 3), true);
  }

  #[test]
  fn test_search_matrix_2() {
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    assert_eq!(search_matrix(matrix, 13), false);
  }

  #[test]
  fn test_search_matrix_3() {
    let matrix = vec![vec![1]];
    assert_eq!(search_matrix(matrix, 1), true);
  }

  #[test]
  fn test_search_matrix_4() {
    let matrix = vec![vec![1]];
    assert_eq!(search_matrix(matrix, 0), false);
  }
}
