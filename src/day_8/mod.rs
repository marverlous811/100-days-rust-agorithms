use std::collections::HashMap;

fn fibonachi(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
  if let Some(&result) = memo.get(&n) {
    return result;
  }
  let result = match n {
    1 => 1,
    2 => 2,
    _ => fibonachi(n - 1, memo) + fibonachi(n - 2, memo),
  };

  memo.insert(n, result);
  result
}

pub fn climb_stairs(n: i32) -> i32 {
  let mut memo = HashMap::new();
  fibonachi(n, &mut memo)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_climb_stairs() {
    assert_eq!(climb_stairs(2), 2);
    assert_eq!(climb_stairs(3), 3);
  }

  #[test]
  fn test_climb_stairs_2() {
    assert_eq!(climb_stairs(45), 1836311903);
  }
}
