use std::{cell::RefCell, rc::Rc};

use crate::common::TreeNode;

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
  match root {
    Some(node) => {
      let node = node.borrow();
      let left = invert_tree(node.left.clone());
      let right = invert_tree(node.right.clone());
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(
        node.val, right, left,
      ))))
    }
    None => None,
  }
}

#[cfg(test)]
mod test {
  use std::{borrow::Borrow, cell::RefCell, rc::Rc};

  use crate::common::TreeNode;

  #[test]
  fn example_1_invert_tree() {
    let mut data = Vec::<i32>::new();
    let root = Rc::new(RefCell::new(TreeNode::new_with_children(
      4,
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(
        2,
        Some(Rc::new(RefCell::new(TreeNode::new_with_children(1, None, None)))),
        Some(Rc::new(RefCell::new(TreeNode::new_with_children(3, None, None)))),
      )))),
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(
        7,
        Some(Rc::new(RefCell::new(TreeNode::new_with_children(6, None, None)))),
        Some(Rc::new(RefCell::new(TreeNode::new_with_children(9, None, None)))),
      )))),
    )));

    let inverted = super::invert_tree(Some(root.clone()));
    if let Some(node) = inverted {
      node.borrow_mut().to_arr(&mut data);
    }
    let expected = vec![9, 7, 6, 4, 3, 2, 1];
    assert_eq!(data, expected);
  }

  #[test]
  fn example_2_invert_tree() {
    let mut data = Vec::<i32>::new();
    let root = Rc::new(RefCell::new(TreeNode::new_with_children(
      2,
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(1, None, None)))),
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(3, None, None)))),
    )));
    let inverted = super::invert_tree(Some(root.clone()));
    if let Some(node) = inverted {
      node.borrow_mut().to_arr(&mut data);
    }
    let expected = vec![3, 2, 1];
    assert_eq!(data, expected);
  }

  #[test]
  fn example_3_invert_tree() {
    let root = None;
    let expected = None;
    assert_eq!(super::invert_tree(root), expected);
  }
}
