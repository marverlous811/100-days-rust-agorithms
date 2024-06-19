pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
  let m = grid.len();
  let n = grid[0].len();

  let direction = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
  let mut visited = vec![vec![0; n]; m];
  let mut queue = std::collections::VecDeque::<((i32, i32), i32)>::new();
  let mut fresh_cnt = 0;
  for (i, row) in grid.iter().enumerate() {
    for (j, col) in row.iter().enumerate() {
      if *col == 2 {
        queue.push_back(((i as i32, j as i32), 0));
        visited[i][j] = 2;
      } else if *col == 1 {
        fresh_cnt += 1;
        visited[i][j] = 1;
      }
    }
  }

  let mut tm = 0;
  while !queue.is_empty() {
    let ((x, y), t) = queue.pop_front().unwrap();
    tm = std::cmp::max(tm, t);

    for (dx, dy) in direction.iter() {
      let nx = x + dx;
      let ny = y + dy;
      if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 && visited[nx as usize][ny as usize] == 1 {
        visited[nx as usize][ny as usize] = 2;
        queue.push_back(((nx, ny), t + 1));
        fresh_cnt -= 1;
      }
    }
  }

  if fresh_cnt == 0 {
    tm
  } else {
    -1
  }
}

#[cfg(test)]
mod test {
  #[test]
  fn example_1() {
    let grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
    assert_eq!(4, super::oranges_rotting(grid));
  }

  #[test]
  fn example_2() {
    let grid = vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]];
    assert_eq!(-1, super::oranges_rotting(grid));
  }

  #[test]
  fn example_3() {
    let grid = vec![vec![0, 2]];
    assert_eq!(0, super::oranges_rotting(grid));
  }
}
