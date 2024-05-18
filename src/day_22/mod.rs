pub fn parenthesis_dfs(left: i32, right: i32, s: &mut String, res: &mut Vec<String>) {
  if left == 0 && right == 0 {
    res.push(s.to_string());
  }

  if left > right || left < 0 || right < 0 {
    return;
  }

  s.push_str(&String::from("("));
  parenthesis_dfs(left - 1, right, s, res);
  s.pop();

  s.push_str(&String::from(")"));
  parenthesis_dfs(left, right - 1, s, res);
  s.pop();
}

pub fn generate_parenthesis(n: i32) -> Vec<String> {
  let mut retval = Vec::<String>::new();
  let mut s = String::new();
  parenthesis_dfs(n, n, &mut s, &mut retval);

  retval
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_generate_parenthesis() {
    assert_eq!(
      generate_parenthesis(3),
      vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
    );
    assert_eq!(generate_parenthesis(1), vec!["()"]);
  }
}
