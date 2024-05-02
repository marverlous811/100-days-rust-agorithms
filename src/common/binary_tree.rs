use std::{borrow::Borrow, cell::RefCell, clone, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

fn array_to_bst(nums: Vec<i32>, start: usize, end: usize) -> Option<Rc<RefCell<TreeNode>>> {
  if start >= end {
    None
  } else {
    let mid = (start + end) / 2;
    Some(Rc::new(RefCell::new(TreeNode::new_with_children(
      nums[mid],
      array_to_bst(nums.clone(), start, mid),
      array_to_bst(nums.clone(), mid + 1, end),
    ))))
  }
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    Self {
      val,
      left: None,
      right: None,
    }
  }

  pub fn new_with_children(
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
  ) -> Self {
    Self { val, left, right }
  }

  pub fn from_arr(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    array_to_bst(nums.clone(), 0, nums.clone().len())
  }

  pub fn to_arr(&self, data: &mut Vec<i32>) {
    if let Some(node) = self.left.clone() {
      node.borrow_mut().to_arr(data);
    }
    data.push(self.val);
    if let Some(node) = self.right.clone() {
      node.borrow_mut().to_arr(data);
    }
  }
}
