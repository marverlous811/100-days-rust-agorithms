use std::{cell::RefCell, rc::Rc};

use crate::common::LinkedListNode;

pub fn reserve_linked_list(root: Option<Rc<RefCell<LinkedListNode>>>) -> Option<Rc<RefCell<LinkedListNode>>> {
  match root {
    Some(node) => {
      let mut curr = Some(node);
      // let mut next = None;
      let mut prev: Option<Rc<RefCell<LinkedListNode>>> = None;
      loop {
        if let Some(curr_node) = curr.clone() {
          let next = curr_node.borrow().next.clone();
          if let Some(prev_node) = prev.clone() {
            curr_node.borrow_mut().next = Some(prev_node.clone());
          } else {
            curr_node.borrow_mut().next = None;
          }

          curr = next.clone();
          prev = Some(curr_node.clone());
        } else {
          break;
        }
      }
      prev
    }
    None => None,
  }
}

#[cfg(test)]
mod test {
  use crate::common::LinkedListNode;

  #[test]
  fn reserve_linked_list_test_1() {
    let root = LinkedListNode::from_vec(vec![1, 2, 3, 4, 5]);
    let exepected = LinkedListNode::from_vec(vec![5, 4, 3, 2, 1]);
    let resvered = super::reserve_linked_list(root);

    assert_eq!(resvered, exepected);
  }

  #[test]
  fn reserve_linked_list_test_2() {
    let root = LinkedListNode::from_vec(vec![1, 2]);
    let exepected = LinkedListNode::from_vec(vec![2, 1]);
    let resvered = super::reserve_linked_list(root);

    assert_eq!(resvered, exepected);
  }

  #[test]
  fn reserve_linked_list_test_3() {
    let root = LinkedListNode::from_vec(vec![]);
    let exepected = LinkedListNode::from_vec(vec![]);
    let resvered = super::reserve_linked_list(root);

    assert_eq!(resvered, exepected);
  }
}
