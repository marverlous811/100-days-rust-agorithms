pub fn longest_palindrome(s: String) -> String {
  let size = s.len();
  let mut start = 0;
  let mut end = 1;

  let mut find_palidrome = |lo: i32, hi: i32| {
    let mut lo = lo;
    let mut hi = hi;
    while lo >= 0 && hi < size as i32 && s.chars().nth(lo as usize).unwrap() == s.chars().nth(hi as usize).unwrap() {
      if hi - lo + 1 > end {
        start = lo;
        end = hi - lo + 1;
      }

      lo -= 1;
      hi += 1;
    }
  };

  for i in 0..size {
    // find even size palidrome
    find_palidrome(i as i32 - 1, i as i32);
    // find odd size palidrome
    find_palidrome(i as i32 - 1, i as i32 + 1);
  }

  s[start as usize..(start + end) as usize].to_string()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn example_1() {
    let res = longest_palindrome("babad".to_string());
    assert!(res.eq("bab") || res.eq("aba"));
    // assert_eq!(longest_palindrome("babad".to_string()), "bab".to_string());
    // assert_eq!(longest_palindrome("cbbd".to_string()), "bb".to_string());
  }

  #[test]
  fn example_2() {
    let res = longest_palindrome("cbbd".to_string());
    assert_eq!(res, "bb".to_string());
  }

  #[test]
  fn exmaple_3() {
    let res = longest_palindrome("a".to_string());
    assert_eq!(res, "a".to_string());
  }

  #[test]
  fn example_4() {
    let res = longest_palindrome("ac".to_string());
    assert_eq!(res, "a".to_string());
  }

  #[test]
  fn example_5() {
    let res = longest_palindrome("babaddtattarrattatddetartrateedredividerb".to_string());
    assert_eq!(res, "ddtattarrattatdd".to_string());
  }
}
