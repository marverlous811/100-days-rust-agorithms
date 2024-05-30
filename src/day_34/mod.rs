use std::borrow::BorrowMut;
use std::collections::HashSet;
use std::{cell::RefCell, rc::Rc};

use crate::common::TreeNode;

static mut PRE_IDX: i32 = 0;

pub fn build_tree_helper(
  preorder: &Vec<i32>,
  inorder: &Vec<i32>,
  in_start: i32,
  in_end: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
  if in_start > in_end {
    return None;
  }

  // if unsafe { PRE_IDX } >= preorder.len() as i32 {
  //   return None;
  // }

  let node = Rc::new(RefCell::new(TreeNode::new(preorder[unsafe { PRE_IDX } as usize])));
  unsafe { PRE_IDX += 1 };

  if unsafe { PRE_IDX } >= preorder.len() as i32 {
    return Some(node);
  }

  if in_start == in_end {
    return Some(node);
  }

  let in_index = inorder.iter().position(|&x| x == node.as_ref().borrow().val).unwrap() as i32;

  node.as_ref().borrow_mut().left = build_tree_helper(preorder, inorder, in_start, in_index - 1);
  node.as_ref().borrow_mut().right = build_tree_helper(preorder, inorder, in_index + 1, in_end);

  Some(node)
}

pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
  unsafe { PRE_IDX = 0 };
  build_tree_helper(&preorder, &inorder, 0, inorder.len() as i32)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_build_tree() {
    let preorder = vec![3, 9, 20, 15, 7];
    let inorder = vec![9, 3, 15, 20, 7];
    let result = build_tree(preorder, inorder);
    let expected = Some(Rc::new(RefCell::new(TreeNode::new_with_children(
      3,
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(9, None, None)))),
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(
        20,
        Some(Rc::new(RefCell::new(TreeNode::new_with_children(15, None, None)))),
        Some(Rc::new(RefCell::new(TreeNode::new_with_children(7, None, None)))),
      )))),
    ))));
    assert_eq!(result, expected);
  }

  #[test]
  fn test_build_tree_2() {
    let preorder = vec![-1];
    let inorder = vec![-1];
    let result = build_tree(preorder, inorder);
    let expected = Some(Rc::new(RefCell::new(TreeNode::new(-1))));
    assert_eq!(result, expected);
  }
}
