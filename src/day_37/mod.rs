use std::{cell::RefCell, rc::Rc};

use crate::common::TreeNode;

fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
  match root {
    Some(node) => {
      // let node = node.borrow();
      inorder_traversal(node.borrow().left.clone(), res);
      res.push(node.borrow().val);
      inorder_traversal(node.borrow().right.clone(), res);
    }
    None => {}
  }
}

pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
  let mut res = vec![];
  inorder_traversal(root, &mut res);
  res[k as usize - 1]
}

#[cfg(test)]
mod test {
  use std::{cell::RefCell, rc::Rc};

  use crate::common::TreeNode;

  #[test]
  fn example_1() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
      val: 3,
      left: Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
          val: 2,
          left: None,
          right: None,
        }))),
      }))),
      right: Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: None,
        right: None,
      }))),
    })));
    let k = 1;
    let expected = 1;
    assert_eq!(super::kth_smallest(root, k), expected);
  }

  #[test]
  fn example_2() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new_with_children(
      5,
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(
        3,
        Some(Rc::new(RefCell::new(TreeNode::new_with_children(
          2,
          Some(Rc::new(RefCell::new(TreeNode::new_with_children(1, None, None)))),
          None,
        )))),
        Some(Rc::new(RefCell::new(TreeNode::new_with_children(4, None, None)))),
      )))),
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(6, None, None)))),
    ))));
    let k = 3;
    let expected = 3;
    assert_eq!(super::kth_smallest(root, k), expected);
  }
}
