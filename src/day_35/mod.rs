use std::{cell::RefCell, rc::Rc};

use crate::common::TreeNode;

fn preorder_traverse(root: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<Rc<RefCell<TreeNode>>>) {
  if let Some(node) = root {
    v.push(node.clone());
    preorder_traverse(node.borrow().left.clone(), v);
    preorder_traverse(node.borrow().right.clone(), v);
  }
}

pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
  if root.is_none() {
    return;
  }

  let mut nodes = vec![];
  preorder_traverse(root.clone(), &mut nodes);
  let n = nodes.len();
  for i in 0..n - 1 {
    nodes[i].as_ref().borrow_mut().left = None;
    nodes[i].as_ref().borrow_mut().right = Some(nodes[i + 1].clone());
  }
}

#[cfg(test)]
mod test {
  use std::{cell::RefCell, rc::Rc};

  use crate::common::TreeNode;

  use super::flatten;

  #[test]
  fn test_flatten() {
    let mut root = Some(Rc::new(RefCell::new(TreeNode::new_with_children(
      1,
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(
        2,
        Some(Rc::new(RefCell::new(TreeNode::new_with_children(3, None, None)))),
        Some(Rc::new(RefCell::new(TreeNode::new_with_children(4, None, None)))),
      )))),
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(
        5,
        None,
        Some(Rc::new(RefCell::new(TreeNode::new_with_children(6, None, None)))),
      )))),
    ))));

    let expected = Some(Rc::new(RefCell::new(TreeNode::new_with_children(
      1,
      None,
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(
        2,
        None,
        Some(Rc::new(RefCell::new(TreeNode::new_with_children(
          3,
          None,
          Some(Rc::new(RefCell::new(TreeNode::new_with_children(
            4,
            None,
            Some(Rc::new(RefCell::new(TreeNode::new_with_children(
              5,
              None,
              Some(Rc::new(RefCell::new(TreeNode::new_with_children(6, None, None)))),
            )))),
          )))),
        )))),
      )))),
    ))));

    flatten(&mut root);
    assert_eq!(root, expected);
  }

  #[test]
  fn test_flatten_2() {
    let mut root: Option<Rc<RefCell<TreeNode>>> = None;
    let expected = None;
    flatten(&mut root);
    assert_eq!(root, expected);
  }

  #[test]
  fn test_flatten_3() {
    let mut root = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    let expected = Some(Rc::new(RefCell::new(TreeNode::new(0))));

    flatten(&mut root);
    assert_eq!(root, expected);
  }
}
