use std::{cell::RefCell, rc::Rc};

use crate::common::TreeNode;

fn is_mirror(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
  match (left, right) {
    (Some(left), Some(right)) => {
      let (left, right) = (left.borrow(), right.borrow());
      if left.val != right.val {
        return false;
      }
      is_mirror(left.left.clone(), right.right.clone()) && is_mirror(left.right.clone(), right.left.clone())
    }
    (None, None) => return true,
    _ => return false,
  }
}

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
  match root {
    Some(root) => {
      let root = root.borrow();
      is_mirror(root.left.clone(), root.right.clone())
    }
    None => true,
  }
}

#[cfg(test)]
mod test {
  use std::{cell::RefCell, rc::Rc};

  use crate::common::TreeNode;

  #[test]
  fn example_1_is_symmetric() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new_with_children(
      1,
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(
        2,
        Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        Some(Rc::new(RefCell::new(TreeNode::new(4)))),
      )))),
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(
        2,
        Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        Some(Rc::new(RefCell::new(TreeNode::new(3)))),
      )))),
    ))));
    let expected = true;
    assert_eq!(super::is_symmetric(root), expected);
  }

  #[test]
  fn example_2_is_symmetric() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new_with_children(
      1,
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(
        2,
        None,
        Some(Rc::new(RefCell::new(TreeNode::new(3)))),
      )))),
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(
        2,
        None,
        Some(Rc::new(RefCell::new(TreeNode::new(3)))),
      )))),
    ))));
    let expected = false;
    assert_eq!(super::is_symmetric(root), expected);
  }
}
