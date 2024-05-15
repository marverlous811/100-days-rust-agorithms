pub fn majority_element(nums: Vec<i32>) -> i32 {
  let bits = 32;
  let mut major = 0;
  for i in 0..bits {
    let mut zeros_bits_count = 0;
    let mut ones_bits_count = 0;
    for n in &nums {
      match (n >> i) & 1 {
        1 => ones_bits_count += 1,
        _ => zeros_bits_count += 1,
      }
    }

    if ones_bits_count > zeros_bits_count {
      major |= 1 << i;
    }
  }

  let mut major_cnt = 0;
  for n in &nums {
    if *n == major {
      major_cnt += 1;
    }
  }
  if major_cnt > nums.len() / 2 {
    major
  } else {
    0
  }
}

fn ez_majority_element(nums: Vec<i32>) -> i32 {
  let mut cnt = 0;
  let mut candidate = 0;
  for n in &nums {
    if cnt == 0 {
      candidate = *n;
    }

    if *n == candidate {
      cnt += 1;
    } else {
      cnt -= 1;
    }
  }

  candidate
}

#[cfg(test)]
mod test {
  #[test]
  fn majority_element_test() {
    assert_eq!(super::majority_element(vec![3, 2, 3]), 3);
    assert_eq!(super::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
  }
}
