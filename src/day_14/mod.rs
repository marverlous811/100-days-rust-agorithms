use std::{cell::RefCell, rc::Rc};

use crate::common::LinkedListNode;

pub fn get_intersection_node(
  head1: Option<Rc<RefCell<LinkedListNode>>>,
  head2: Option<Rc<RefCell<LinkedListNode>>>,
) -> Option<Rc<RefCell<LinkedListNode>>> {
  let mut p1 = head1.clone();
  let mut p2 = head2.clone();
  loop {
    match (p1.clone(), p2.clone()) {
      (Some(node1), Some(node2)) => {
        if Rc::ptr_eq(&node1, &node2) {
          return Some(node1);
        }
        p1 = node1.borrow().next.clone();
        p2 = node2.borrow().next.clone();
      }
      (Some(_node1), None) => {
        p2 = head1.clone();
      }
      (None, Some(_node2)) => {
        p1 = head2.clone();
      }
      _ => {
        break;
      }
    };
  }
  None
}

#[cfg(test)]
mod test {
  use std::{cell::RefCell, rc::Rc};

  use crate::common::LinkedListNode;

  #[test]
  fn get_intersection_node_example_1() {
    // Linked List 1
    let node_1 = Rc::new(RefCell::new(LinkedListNode::new(4)));
    let node_2 = Rc::new(RefCell::new(LinkedListNode::new(1)));
    let node_3 = Rc::new(RefCell::new(LinkedListNode::new(8))); // <- intersection
    let node_4 = Rc::new(RefCell::new(LinkedListNode::new(4)));
    let node_5 = Rc::new(RefCell::new(LinkedListNode::new(5)));

    node_1.borrow_mut().next = Some(node_2.clone());
    node_2.borrow_mut().next = Some(node_3.clone());
    node_3.borrow_mut().next = Some(node_4.clone());
    node_4.borrow_mut().next = Some(node_5.clone());

    // Linked List 2
    let node_6 = Rc::new(RefCell::new(LinkedListNode::new(5)));
    let node_7 = Rc::new(RefCell::new(LinkedListNode::new(6)));

    node_6.borrow_mut().next = Some(node_7.clone());
    node_7.borrow_mut().next = Some(node_3.clone());

    let result = super::get_intersection_node(Some(node_1), Some(node_6));
    assert_eq!(result, Some(node_3));
  }

  #[test]
  fn get_intersection_node_example_2() {
    // Linked List 1
    let node_1 = Rc::new(RefCell::new(LinkedListNode::new(1)));
    let node_2 = Rc::new(RefCell::new(LinkedListNode::new(9)));
    let node_3 = Rc::new(RefCell::new(LinkedListNode::new(1)));
    let node_4 = Rc::new(RefCell::new(LinkedListNode::new(2))); // <- intersection
    let node_5 = Rc::new(RefCell::new(LinkedListNode::new(4)));

    node_1.borrow_mut().next = Some(node_2.clone());
    node_2.borrow_mut().next = Some(node_3.clone());
    node_3.borrow_mut().next = Some(node_4.clone());
    node_4.borrow_mut().next = Some(node_5.clone());

    // Linked List 2
    let node_6 = Rc::new(RefCell::new(LinkedListNode::new(3)));
    node_6.borrow_mut().next = Some(node_4.clone());

    let result = super::get_intersection_node(Some(node_1), Some(node_6));
    assert_eq!(result, Some(node_4));
  }

  #[test]
  fn get_intersection_node_example_3() {
    // Linked List 1
    let node_1 = Rc::new(RefCell::new(LinkedListNode::new(2)));
    let node_2 = Rc::new(RefCell::new(LinkedListNode::new(6)));
    let node_3 = Rc::new(RefCell::new(LinkedListNode::new(4)));

    node_1.borrow_mut().next = Some(node_2.clone());
    node_2.borrow_mut().next = Some(node_3.clone());

    // Linked List 2
    let node_4 = Rc::new(RefCell::new(LinkedListNode::new(1)));
    let node_5 = Rc::new(RefCell::new(LinkedListNode::new(5)));

    node_4.borrow_mut().next = Some(node_5.clone());

    let result = super::get_intersection_node(Some(node_1), Some(node_4));
    assert_eq!(result, None);
  }
}
