pub fn letter_combinations(digits: String) -> Vec<String> {
  let matching_digit = |digit: char| match digit {
    '2' => vec!['a', 'b', 'c'],
    '3' => vec!['d', 'e', 'f'],
    '4' => vec!['g', 'h', 'i'],
    '5' => vec!['j', 'k', 'l'],
    '6' => vec!['m', 'n', 'o'],
    '7' => vec!['p', 'q', 'r', 's'],
    '8' => vec!['t', 'u', 'v'],
    '9' => vec!['w', 'x', 'y', 'z'],
    _ => vec![],
  };

  if digits.len() == 0 {
    return vec![];
  }

  let mut retval = vec![];
  let mut queue = vec!["".to_string()];
  while queue.len() > 0 {
    let current = queue.remove(0);
    if current.len() == digits.len() {
      retval.push(current.clone());
    } else {
      let digit_letters = matching_digit(digits.chars().nth(current.len()).unwrap());
      for letter in digit_letters {
        // current.clone().to_string().push_str(&letter.to_string());
        let mut new_str = current.clone();
        new_str.push_str(&letter.to_string());
        queue.push(new_str);
      }
    }
  }
  retval
}

#[cfg(test)]
mod test {
  #[test]
  fn test_letter_combinations() {
    assert_eq!(
      super::letter_combinations("23".to_string()),
      vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
    );
    assert_eq!(super::letter_combinations("".to_string()), vec![] as Vec<String>);
    assert_eq!(super::letter_combinations("2".to_string()), vec!["a", "b", "c"]);
  }
}
