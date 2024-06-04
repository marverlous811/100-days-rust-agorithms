use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::common::TreeNode;

pub fn sum_traversal(root: Option<Rc<RefCell<TreeNode>>>, target: i64, sum: &mut i64, res: &mut i32) {
  if let Some(node) = root {
    let curr = node.borrow().val;

    *sum += curr as i64;
    if *sum == target {
      *res += 1;
    }

    sum_traversal(node.borrow().left.clone(), target, sum, res);
    sum_traversal(node.borrow().right.clone(), target, sum, res);

    *sum -= curr as i64;
  }
}

pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
  let mut res = 0;
  let mut stack = vec![root];

  while stack.len() > 0 {
    let node = stack.pop().unwrap();
    if let Some(node) = node.clone() {
      stack.push(node.borrow().left.clone());
      stack.push(node.borrow().right.clone());
    }

    sum_traversal(node.clone(), target_sum as i64, &mut 0, &mut res);
  }

  res
}

#[cfg(test)]
mod test {
  use std::{cell::RefCell, rc::Rc};

  use crate::common::TreeNode;

  #[test]
  fn example_1() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new_with_children(
      10,
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(
        5,
        Some(Rc::new(RefCell::new(TreeNode::new_with_children(
          3,
          Some(Rc::new(RefCell::new(TreeNode::new_with_children(3, None, None)))),
          Some(Rc::new(RefCell::new(TreeNode::new_with_children(-2, None, None)))),
        )))),
        Some(Rc::new(RefCell::new(TreeNode::new_with_children(
          2,
          None,
          Some(Rc::new(RefCell::new(TreeNode::new_with_children(1, None, None)))),
        )))),
      )))),
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(
        -3,
        None,
        Some(Rc::new(RefCell::new(TreeNode::new_with_children(11, None, None)))),
      )))),
    ))));

    assert_eq!(super::path_sum(root, 8), 3);
  }

  #[test]
  fn example_2() {
    let tree = Some(Rc::new(RefCell::new(TreeNode::new_with_children(
      5,
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(
        4,
        Some(Rc::new(RefCell::new(TreeNode::new_with_children(
          11,
          Some(Rc::new(RefCell::new(TreeNode::new_with_children(7, None, None)))),
          Some(Rc::new(RefCell::new(TreeNode::new_with_children(2, None, None)))),
        )))),
        None,
      )))),
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(
        8,
        Some(Rc::new(RefCell::new(TreeNode::new_with_children(13, None, None)))),
        Some(Rc::new(RefCell::new(TreeNode::new_with_children(
          4,
          Some(Rc::new(RefCell::new(TreeNode::new_with_children(5, None, None)))),
          Some(Rc::new(RefCell::new(TreeNode::new_with_children(1, None, None)))),
        )))),
      )))),
    ))));

    assert_eq!(super::path_sum(tree, 22), 3);
  }

  #[test]
  fn example_3() {
    let tree = Some(Rc::new(RefCell::new(TreeNode::new_with_children(
      1000000000,
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(
        1000000000,
        Some(Rc::new(RefCell::new(TreeNode::new_with_children(
          294967296,
          Some(Rc::new(RefCell::new(TreeNode::new_with_children(
            1000000000,
            Some(Rc::new(RefCell::new(TreeNode::new_with_children(
              1000000000,
              Some(Rc::new(RefCell::new(TreeNode::new_with_children(
                1000000000, None, None,
              )))),
              None,
            )))),
            None,
          )))),
          None,
        )))),
        None,
      )))),
      None,
    ))));

    assert_eq!(super::path_sum(tree, 0), 0);
  }
}
