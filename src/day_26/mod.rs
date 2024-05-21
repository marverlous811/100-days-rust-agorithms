fn backtrack(
  board: &mut Vec<Vec<char>>,
  rows: usize,
  columns: usize,
  target: Vec<char>,
  idx: usize,
  x: i32,
  y: i32,
) -> bool {
  if idx == target.len() {
    return true;
  }

  if x < 0 || x >= rows as i32 || y < 0 || y >= columns as i32 {
    return false;
  }

  if board[x as usize][y as usize] == target[idx] {
    let directory = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let temp = board[x as usize][y as usize];

    board[x as usize][y as usize] = '0';

    for (dx, dy) in directory {
      let new_x = x + dx;
      let new_y = y + dy;

      if backtrack(board, rows, columns, target.clone(), idx + 1, new_x, new_y) {
        return true;
      }
    }

    board[x as usize][y as usize] = temp;
  }

  if idx == 0 {
    let mut next_x = x;
    let mut next_y = y + 1;
    if next_y >= columns as i32 {
      next_x += 1;
      next_y = 0;
    }

    backtrack(board, rows, columns, target, idx, next_x, next_y)
  } else {
    false
  }
}

pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
  let mut target = word.clone().chars().collect::<Vec<char>>();
  let mut board = board.clone();
  let row = board.len();
  let column = board[0].len();

  backtrack(&mut board, row, column, target, 0, 0, 0)
}

#[cfg(test)]
mod test {
  #[test]
  fn test_exist_1() {
    assert_eq!(
      super::exist(
        vec![
          vec!['A', 'B', 'C', 'E'],
          vec!['S', 'F', 'C', 'S'],
          vec!['A', 'D', 'E', 'E']
        ],
        "ABCCED".to_string()
      ),
      true
    );
  }

  #[test]
  fn test_exist_2() {
    assert_eq!(
      super::exist(
        vec![
          vec!['A', 'B', 'C', 'E'],
          vec!['S', 'F', 'C', 'S'],
          vec!['A', 'D', 'E', 'E']
        ],
        "SEE".to_string()
      ),
      true
    );
  }

  #[test]
  fn test_exist_3() {
    assert_eq!(
      super::exist(
        vec![
          vec!['A', 'B', 'C', 'E'],
          vec!['S', 'F', 'C', 'S'],
          vec!['A', 'D', 'E', 'E']
        ],
        "ABCB".to_string()
      ),
      false
    );
  }

  #[test]
  fn test_exist_4() {
    assert_eq!(
      super::exist(vec![vec!['A', 'B'], vec!['C', 'D'],], "ABCD".to_string()),
      false
    );
  }
}
