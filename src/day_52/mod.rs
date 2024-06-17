fn is_safe(grid: &Vec<Vec<char>>, row: i32, col: i32, visited: &mut Vec<Vec<bool>>) -> bool {
  row >= 0
    && row < grid.len() as i32
    && col >= 0
    && col < grid[0].len() as i32
    && grid[row as usize][col as usize] == '1'
    && !visited[row as usize][col as usize]
}

fn dfs(grid: &Vec<Vec<char>>, row: i32, col: i32, visited: &mut Vec<Vec<bool>>) {
  let director = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
  visited[row as usize][col as usize] = true;

  for (dx, dy) in director {
    if is_safe(grid, row + dx, col + dy, visited) {
      dfs(grid, row + dx, col + dy, visited);
    }
  }
}

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
  let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
  let mut count = 0;
  for (i, _) in grid.iter().enumerate() {
    for (j, _) in grid[i].iter().enumerate() {
      if grid[i][j] == '1' && !visited[i][j] {
        dfs(&grid, i as i32, j as i32, &mut visited);
        count += 1;
      }
    }
  }
  count
}

#[cfg(test)]
mod test {
  #[test]
  fn example_1() {
    let grid = vec![
      vec!['1', '1', '1', '1', '0'],
      vec!['1', '1', '0', '1', '0'],
      vec!['1', '1', '0', '0', '0'],
      vec!['0', '0', '0', '0', '0'],
    ];
    assert_eq!(super::num_islands(grid), 1);
  }

  #[test]
  fn example_2() {
    let grid = vec![
      vec!['1', '1', '0', '0', '0'],
      vec!['1', '1', '0', '0', '0'],
      vec!['0', '0', '1', '0', '0'],
      vec!['0', '0', '0', '1', '1'],
    ];
    assert_eq!(super::num_islands(grid), 3);
  }

  #[test]
  fn example_3() {
    let grid = vec![vec!['1'], vec!['1']];
    assert_eq!(super::num_islands(grid), 1);
  }
}
