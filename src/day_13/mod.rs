use std::cell::RefCell;

use crate::common::ListNode;

pub fn has_cycle(root: Option<Box<ListNode>>) -> bool {
  let (mut slow, mut fast) = (&root, &root);
  // println!("slow: {:?}, fast: {:?}", slow, fast);
  loop {
    match (slow, fast) {
      (Some(slow_p), Some(fast_p)) => {
        // println!("slow: {}, fast: {}", slow_p.val, fast_p.val);
        slow = &slow_p.next;
        let _ = &fast_p.next.as_ref().map(|node| {
          fast = &node.next;
        });
        if slow == fast {
          return true;
        }
        // println!("slow: {:?}, fast: {:?}", slow, fast);
      }
      _ => {
        // println!("???");
        break;
      }
    }
  }
  false
}

#[cfg(test)]
mod test {
  use crate::common::ListNode;

  use super::has_cycle;
  #[test]
  fn has_cycle_example_1() {
    let mut cycle_node = Box::new(ListNode::new(2));
    let repeated_node = Box::new(ListNode::new_with_next(4, Some(cycle_node.clone())));
    cycle_node.add_node(Some(Box::new(ListNode::new_with_next(0, Some(repeated_node.clone())))));
    let root = ListNode::new_with_next(3, Some(cycle_node));

    assert_eq!(has_cycle(Some(Box::new(root))), true);
  }

  #[test]
  fn has_cycle_example_3() {
    let root = ListNode::new(1);
    assert_eq!(has_cycle(Some(Box::new(root))), false);
  }
}
