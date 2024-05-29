use std::{cell::RefCell, rc::Rc};

use crate::common::TreeNode;

fn dfs_level_travesal(root: Option<Rc<RefCell<TreeNode>>>, level: i32, res: &mut Vec<Vec<i32>>) {
  match root {
    Some(node) => {
      let cur = node.borrow();
      if res.len() <= level as usize {
        res.push(vec![]);
      }
      res[level as usize].push(cur.val);

      dfs_level_travesal(cur.left.clone(), level + 1, res);
      dfs_level_travesal(cur.right.clone(), level + 1, res);
    }
    None => {}
  }
}

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
  let mut res = vec![];
  dfs_level_travesal(root, 0, &mut res);
  res
}

#[cfg(test)]
mod test {
  use std::{cell::RefCell, rc::Rc};

  use crate::common::TreeNode;

  #[test]
  fn test_level_order() {
    let tree = Rc::new(RefCell::new(TreeNode::new_with_children(
      3,
      Some(Rc::new(RefCell::new(TreeNode::new(9)))),
      Some(Rc::new(RefCell::new(TreeNode::new_with_children(
        20,
        Some(Rc::new(RefCell::new(TreeNode::new(15)))),
        Some(Rc::new(RefCell::new(TreeNode::new(7)))),
      )))),
    )));
    assert_eq!(super::level_order(Some(tree)), vec![vec![3], vec![9, 20], vec![15, 7]]);
  }

  #[test]
  fn test_level_order_2() {
    let tree = Rc::new(RefCell::new(TreeNode::new(1)));
    assert_eq!(super::level_order(Some(tree)), vec![vec![1]]);
  }

  #[test]
  fn test_level_order_3() {
    assert_eq!(super::level_order(None), vec![] as Vec<Vec<i32>>);
  }
}
