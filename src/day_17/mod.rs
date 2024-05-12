pub fn is_valid(s: String) -> bool {
  let mut val = -1;
  let mut i = 0;
  let mut stack = vec![];
  while i < s.len() {
    let cur = s.chars().nth(i).unwrap();
    if cur == '[' || cur == '(' || cur == '{' {
      stack.push(cur);
      val += 1;
    } else {
      if val >= 0
        && ((stack[val as usize] == '[' && cur == ']')
          || (stack[val as usize] == '(' && cur == ')')
          || (stack[val as usize] == '{' && cur == '}'))
      {
        stack.pop();
        val -= 1;
      } else {
        return false;
      }
    }
    i += 1;
  }

  val == -1
}

#[cfg(test)]
mod test {
  #[test]
  fn test_is_valid() {
    assert_eq!(super::is_valid("()".to_string()), true);
    assert_eq!(super::is_valid("()[]{}".to_string()), true);
    assert_eq!(super::is_valid("(]".to_string()), false);
    assert_eq!(super::is_valid("([)]".to_string()), false);
    assert_eq!(super::is_valid("{[]}".to_string()), true);
    assert_eq!(super::is_valid("[".to_string()), false);
    assert_eq!(super::is_valid("(([]){})".to_string()), true);
    assert_eq!(super::is_valid("{}{}()[]".to_string()), true);
  }
}
