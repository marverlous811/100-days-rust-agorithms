use std::{cell::RefCell, rc::Rc};

use crate::common::TreeNode;

pub fn right_side_view_tranversal(root: Option<Rc<RefCell<TreeNode>>>, level: usize, result: &mut Vec<i32>) {
  if let Some(node) = root {
    if result.len() == level {
      result.push(node.borrow().val);
    }

    right_side_view_tranversal(node.borrow().right.clone(), level + 1, result);
    right_side_view_tranversal(node.borrow().left.clone(), level + 1, result);
  }
}

pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
  let mut res = vec![];
  right_side_view_tranversal(root, 0, &mut res);
  res
}

#[cfg(test)]
mod test {
  use std::{cell::RefCell, rc::Rc};

  use crate::common::TreeNode;

  #[test]
  fn example_1() {
    let tree = Rc::new(RefCell::new(TreeNode::new_with_children(
      1,
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(
        2,
        None,
        Some(Rc::new(RefCell::new(TreeNode::new_with_children(5, None, None)))),
      )))),
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(
        3,
        None,
        Some(Rc::new(RefCell::new(TreeNode::new_with_children(4, None, None)))),
      )))),
    )));

    let expected = vec![1, 3, 4];
    assert_eq!(super::right_side_view(Some(tree)), expected);
  }

  #[test]
  fn example_2() {
    let tree = Rc::new(RefCell::new(TreeNode::new_with_children(
      1,
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(
        2,
        None,
        Some(Rc::new(RefCell::new(TreeNode::new_with_children(5, None, None)))),
      )))),
      None,
    )));

    let expected = vec![1, 2, 5];
    assert_eq!(super::right_side_view(Some(tree)), expected);
  }

  #[test]
  fn example_3() {
    let tree = Rc::new(RefCell::new(TreeNode::new_with_children(
      1,
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(
        2,
        None,
        Some(Rc::new(RefCell::new(TreeNode::new_with_children(5, None, None)))),
      )))),
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(3, None, None)))),
    )));

    let expected = vec![1, 3, 5];
    assert_eq!(super::right_side_view(Some(tree)), expected);
  }

  #[test]
  fn example_4() {
    let tree = None;
    let expected = vec![];

    assert_eq!(super::right_side_view(tree), expected);
  }
}
