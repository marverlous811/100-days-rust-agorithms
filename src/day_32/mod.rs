use std::{cell::RefCell, rc::Rc};

use crate::common::TreeNode;

fn bst_dfs_validate(root: Option<Rc<RefCell<TreeNode>>>, left: Option<i32>, right: Option<i32>) -> bool {
  match root {
    Some(node) => {
      let node = node.borrow();
      // println!("left: {}, right: {}, node: {}", left, right, node.val);
      if let Some(min) = left {
        if node.val <= min {
          return false;
        }
      }
      if let Some(max) = right {
        if node.val >= max {
          return false;
        }
      }

      bst_dfs_validate(node.left.clone(), left, Some(node.val))
        && bst_dfs_validate(node.right.clone(), Some(node.val), right)
    }
    None => true,
  }
}

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
  bst_dfs_validate(root, None, None)
}

#[cfg(test)]
mod test {
  use std::{cell::RefCell, rc::Rc};

  use crate::common::TreeNode;

  #[test]
  fn test_is_valid_bst() {
    let tree = Rc::new(RefCell::new(TreeNode::new_with_children(
      2,
      Some(Rc::new(RefCell::new(TreeNode::new(1)))),
      Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    )));
    assert_eq!(super::is_valid_bst(Some(tree)), true);
  }

  #[test]
  fn test_is_valid_bst_2() {
    let tree = Rc::new(RefCell::new(TreeNode::new_with_children(
      5,
      Some(Rc::new(RefCell::new(TreeNode::new(1)))),
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(
        4,
        Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        Some(Rc::new(RefCell::new(TreeNode::new(6)))),
      )))),
    )));
    assert_eq!(super::is_valid_bst(Some(tree)), false);
  }

  #[test]
  fn test_is_valid_bst_3() {
    let tree = Rc::new(RefCell::new(TreeNode::new_with_children(
      5,
      Some(Rc::new(RefCell::new(TreeNode::new(4)))),
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(
        6,
        Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        Some(Rc::new(RefCell::new(TreeNode::new(7)))),
      )))),
    )));
    assert_eq!(super::is_valid_bst(Some(tree)), false);
  }

  #[test]
  fn test_is_valid_bst_4() {
    let tree = Rc::new(RefCell::new(TreeNode::new(2147483647)));
    assert_eq!(super::is_valid_bst(Some(tree)), true);
  }

  #[test]
  fn test_is_valid_bst_5() {
    let tree = Rc::new(RefCell::new(TreeNode::new_with_children(
      2,
      Some(Rc::new(RefCell::new(TreeNode::new(2)))),
      Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    )));
    assert_eq!(super::is_valid_bst(Some(tree)), false);
  }
}
