use std::{cell::RefCell, rc::Rc};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LinkedListNode {
  pub val: i32,
  pub next: Option<Rc<RefCell<LinkedListNode>>>,
}

impl LinkedListNode {
  pub fn new(val: i32) -> Self {
    Self { val, next: None }
  }

  pub fn new_with_next(val: i32, next: Option<Rc<RefCell<LinkedListNode>>>) -> Self {
    Self { val, next }
  }

  pub fn from_vec(vals: Vec<i32>) -> Option<Rc<RefCell<LinkedListNode>>> {
    if vals.is_empty() {
      return None;
    } else {
      let head = Rc::new(RefCell::new(LinkedListNode::new(vals[0])));
      let mut current = head.clone();
      for i in 1..vals.len() {
        let new_node = Rc::new(RefCell::new(LinkedListNode::new(vals[i])));
        current.borrow_mut().next = Some(new_node.clone());
        current = new_node;
      }

      Some(head)
    }
  }
}
