use std::vec;

use crate::common::ListNode;

pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
  match head {
    Some(root) => {
      let mut vec = root.clone().to_vec();

      let mut cur = Some(root);
      while let Some(node) = cur {
        if node.val != vec.pop().unwrap() {
          return false;
        }
        cur = node.next;
      }

      true
    }
    None => return false,
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_is_palindrome_1() {
    let input = ListNode::from_vec(vec![1, 2, 2, 1]);
    assert_eq!(is_palindrome(input), true);
  }

  #[test]
  fn test_is_palindrome_2() {
    let input = ListNode::from_vec(vec![1, 2]);
    assert_eq!(is_palindrome(input), false);
  }
}
