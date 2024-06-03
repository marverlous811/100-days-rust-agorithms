use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use crate::common::TreeNode;

fn find_path(
  root: Option<Rc<RefCell<TreeNode>>>,
  path: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
  target: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
  match (root, target) {
    (Some(node), Some(t)) => {
      path.push(Some(node.clone()));
      if node.as_ref().borrow().val == t.as_ref().borrow().val {
        return true;
      }

      if find_path(node.as_ref().borrow().left.clone(), path, Some(t.clone()))
        || find_path(node.as_ref().borrow().right.clone(), path, Some(t.clone()))
      {
        return true;
      }

      path.pop();
      false
    }
    _ => false,
  }
}

pub fn debug_print(test: &Vec<Option<Rc<RefCell<TreeNode>>>>) {
  let mut res = vec![];
  for node in test {
    match node {
      Some(n) => {
        res.push(n.as_ref().borrow().val);
      }
      None => {
        // println!("None");
      }
    }
  }

  println!("{:?}", res);
}

pub fn lowest_common_ancestor(
  root: Option<Rc<RefCell<TreeNode>>>,
  p: Option<Rc<RefCell<TreeNode>>>,
  q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
  let mut path_1 = vec![];
  let mut path_2 = vec![];

  if !find_path(root.clone(), &mut path_1, q.clone()) || !find_path(root.clone(), &mut path_2, p.clone()) {
    return None;
  }

  let mut i = 0;
  loop {
    if i > path_1.len() - 1 || i > path_2.len() - 1 {
      break;
    }

    if path_1[i] != path_2[i] {
      break;
    }
    i += 1;
  }

  path_1[i - 1].clone()
}

#[cfg(test)]
mod test {
  use std::{cell::RefCell, rc::Rc};

  use crate::common::TreeNode;

  #[test]
  fn example_1() {
    let mut node_1 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let mut node_2 = Some(Rc::new(RefCell::new(TreeNode::new(5)))); // <- p
    let mut node_3 = Some(Rc::new(RefCell::new(TreeNode::new(1)))); // <- q
    let node_4 = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    let mut node_5 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let node_6 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    let node_7 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    let node_8 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    let node_9 = Some(Rc::new(RefCell::new(TreeNode::new(8))));

    node_5.as_mut().unwrap().borrow_mut().left = node_7.clone();
    node_5.as_mut().unwrap().borrow_mut().right = node_6.clone();
    node_2.as_mut().unwrap().borrow_mut().left = node_4.clone();
    node_2.as_mut().unwrap().borrow_mut().right = node_5.clone();
    node_3.as_mut().unwrap().borrow_mut().left = node_8.clone();
    node_3.as_mut().unwrap().borrow_mut().right = node_9.clone();
    node_1.as_mut().unwrap().borrow_mut().left = node_2.clone();
    node_1.as_mut().unwrap().borrow_mut().right = node_3.clone();

    let result = super::lowest_common_ancestor(node_1.clone(), node_2.clone(), node_3.clone());
    assert_eq!(result, node_1);
  }

  #[test]
  fn example_2() {
    let mut node_1 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let mut node_2 = Some(Rc::new(RefCell::new(TreeNode::new(5)))); // <-- p
    let mut node_3 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let node_4 = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    let mut node_5 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let node_6 = Some(Rc::new(RefCell::new(TreeNode::new(4)))); // <-- q
    let node_7 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    let node_8 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    let node_9 = Some(Rc::new(RefCell::new(TreeNode::new(8))));

    node_5.as_mut().unwrap().borrow_mut().left = node_7.clone();
    node_5.as_mut().unwrap().borrow_mut().right = node_6.clone();
    node_2.as_mut().unwrap().borrow_mut().left = node_4.clone();
    node_2.as_mut().unwrap().borrow_mut().right = node_5.clone();
    node_3.as_mut().unwrap().borrow_mut().left = node_8.clone();
    node_3.as_mut().unwrap().borrow_mut().right = node_9.clone();
    node_1.as_mut().unwrap().borrow_mut().left = node_2.clone();
    node_1.as_mut().unwrap().borrow_mut().right = node_3.clone();

    let result = super::lowest_common_ancestor(node_1.clone(), node_2.clone(), node_6.clone());
    assert_eq!(result, node_2);
  }

  #[test]
  fn example_3() {
    let mut node_1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let node_2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    node_1.as_mut().unwrap().borrow_mut().left = node_2.clone();
    let result = super::lowest_common_ancestor(node_1.clone(), node_1.clone(), node_2.clone());
    assert_eq!(result, node_1);
  }
}
