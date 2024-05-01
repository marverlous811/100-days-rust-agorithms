use std::{cell::RefCell, rc::Rc};

use crate::common::TreeNode;

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
  match root {
    Some(node) => {
      let node = node.borrow();
      let left_len = max_depth(node.left.clone());
      let right_len = max_depth(node.right.clone());
      1 + left_len.max(right_len)
    }
    None => 0,
  }
}

#[cfg(test)]
mod test {
  use std::{cell::RefCell, rc::Rc};

  use crate::common::TreeNode;

  #[test]
  fn example_1_max_depth() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new_with_children(
      3,
      Some(Rc::new(RefCell::new(TreeNode::new(9)))),
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(
        20,
        Some(Rc::new(RefCell::new(TreeNode::new(15)))),
        Some(Rc::new(RefCell::new(TreeNode::new(7)))),
      )))),
    ))));
    let expected = 3;
    assert_eq!(super::max_depth(root), expected);
  }

  #[test]
  fn example_2_max_depth() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new_with_children(
      1,
      None,
      Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    ))));
    let expected = 2;
    assert_eq!(super::max_depth(root), expected);
  }
}
