use std::{cell::RefCell, rc::Rc};

use crate::common::TreeNode;

fn sorted_array_to_bst_helper(nums: Vec<i32>, start: usize, end: usize) -> Option<Rc<RefCell<TreeNode>>> {
  if start >= end {
    None
  } else {
    let mid = (start + end) / 2;
    Some(Rc::new(RefCell::new(TreeNode::new_with_children(
      nums[mid],
      sorted_array_to_bst_helper(nums.clone(), start, mid),
      sorted_array_to_bst_helper(nums.clone(), mid + 1, end),
    ))))
  }
}

pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
  sorted_array_to_bst_helper(nums.clone(), 0, nums.clone().len())
}

#[cfg(test)]
mod test {
  use std::{cell::RefCell, rc::Rc};

  use crate::common::TreeNode;

  #[test]
  fn example_1_sorted_array_to_bst() {
    let nums = vec![-10, -3, 0, 5, 9];
    let expected = Some(Rc::new(RefCell::new(TreeNode::new_with_children(
      0,
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(
        -3,
        Some(Rc::new(RefCell::new(TreeNode::new(-10)))),
        None,
      )))),
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(
        9,
        Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        None,
      )))),
    ))));
    assert_eq!(super::sorted_array_to_bst(nums), expected);
  }

  #[test]
  fn example_2_sorted_array_to_bst() {
    let nums = vec![1, 3];
    let expected = Some(Rc::new(RefCell::new(TreeNode::new_with_children(
      3,
      Some(Rc::new(RefCell::new(TreeNode::new(1)))),
      None,
    ))));
    assert_eq!(super::sorted_array_to_bst(nums), expected);
  }
}
