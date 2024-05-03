use std::{cell::RefCell, cmp, rc::Rc};

use crate::common::TreeNode;

pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
  fn helper(root: &Option<Rc<RefCell<TreeNode>>>, max_diameter: &mut i32) -> i32 {
    match root {
      Some(node) => {
        let node = node.borrow();
        let left_height = helper(&node.left, max_diameter);
        let right_height = helper(&node.right, max_diameter);
        *max_diameter = std::cmp::max(*max_diameter, left_height + right_height);
        cmp::max(left_height, right_height) + 1
      }
      None => 0,
    }
  }

  let mut max_diameter = 0;
  helper(&root, &mut max_diameter);
  max_diameter
}

#[cfg(test)]
mod test {
  use std::{cell::RefCell, rc::Rc};

  use crate::common::TreeNode;

  #[test]
  fn example_1() {
    let input = Some(Rc::new(RefCell::new(TreeNode::new_with_children(
      1,
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(
        2,
        Some(Rc::new(RefCell::new(TreeNode::new_with_children(4, None, None)))),
        Some(Rc::new(RefCell::new(TreeNode::new_with_children(5, None, None)))),
      )))),
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(3, None, None)))),
    ))));
    let expected = 3;
    assert_eq!(super::diameter_of_binary_tree(input), expected);
  }

  #[test]
  fn example_2() {
    let input = Some(Rc::new(RefCell::new(TreeNode::new_with_children(
      1,
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(2, None, None)))),
      None,
    ))));
    let expected = 1;
    assert_eq!(super::diameter_of_binary_tree(input), expected);
  }
}
