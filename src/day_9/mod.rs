pub fn next_vec(pre_vec: Vec<i32>) -> Vec<i32> {
  let mut next_vec = vec![1];
  for i in 0..pre_vec.len() - 1 {
    next_vec.push(pre_vec[i] + pre_vec[i + 1]);
  }
  next_vec.push(1);
  next_vec
}

pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
  let mut result = vec![];
  for i in 1..num_rows + 1 {
    match i {
      1 => {
        result.push(vec![1]);
      }
      2 => {
        result.push(vec![1, 1]);
      }
      _ => {
        let pre_vec = result.last().unwrap().to_vec();
        result.push(next_vec(pre_vec));
      }
    };
  }
  result
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_1_generate() {
    assert_eq!(
      generate(5),
      [
        [1].to_vec(),
        [1, 1].to_vec(),
        [1, 2, 1].to_vec(),
        [1, 3, 3, 1].to_vec(),
        [1, 4, 6, 4, 1].to_vec()
      ]
      .to_vec()
    );
  }

  #[test]
  fn test_2_generate() {
    assert_eq!(generate(1), [[1].to_vec()].to_vec());
  }
}
