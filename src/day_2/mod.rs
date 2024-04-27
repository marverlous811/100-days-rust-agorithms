use std::{cell::RefCell, rc::Rc};

use crate::common::TreeNode;

fn inorder_traversal_vec(root: Option<Rc<RefCell<TreeNode>>>, data: &mut Vec<i32>) {
  if let Some(node) = root {
    inorder_traversal_vec(node.borrow().left.clone(), data);
    data.push(node.borrow().val);
    inorder_traversal_vec(node.borrow().right.clone(), data);
  }
}

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
  let mut ret_val = Vec::<i32>::new();

  inorder_traversal_vec(root, &mut ret_val);

  ret_val
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn example_1_inorder_traversal() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new_with_children(
      1,
      None,
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(
        2,
        Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        None,
      )))),
    ))));
    let expected = vec![1, 3, 2];
    assert_eq!(inorder_traversal(root), expected);
  }

  #[test]
  fn example_2_inorder_traversal() {
    let root = None;
    let expected = vec![];
    assert_eq!(inorder_traversal(root), expected);
  }

  #[test]
  fn example_3_inorder_traversal() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let expected = vec![1];
    assert_eq!(inorder_traversal(root), expected);
  }
}
