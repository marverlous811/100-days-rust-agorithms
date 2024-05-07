use crate::common::ListNode;

pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  let mut retval: Option<Box<ListNode>> = None;
  let mut left_node = &list1;
  let mut right_node = &list2;
  let mut current = &mut retval;
  loop {
    match (left_node, right_node) {
      (Some(left), Some(right)) => {
        if left.val < right.val {
          *current = Some(Box::new(ListNode::new(left.val)));
          current = &mut current.as_mut().unwrap().next;

          left_node = &left.next;
        } else {
          *current = Some(Box::new(ListNode::new(right.val)));
          current = &mut current.as_mut().unwrap().next;

          right_node = &right.next;
        }
      }
      (Some(left), None) => {
        *current = Some(Box::new(ListNode::new(left.val)));
        current = &mut current.as_mut().unwrap().next;

        left_node = &left.next;
      }
      (None, Some(right)) => {
        *current = Some(Box::new(ListNode::new(right.val)));
        current = &mut current.as_mut().unwrap().next;

        right_node = &right.next;
      }
      (None, None) => {
        break;
      }
    };
  }

  retval
}

#[cfg(test)]
pub mod test {

  use super::merge_two_lists;
  use crate::common::ListNode;
  #[test]
  fn merge_two_lists_example_1() {
    let list1 = ListNode::from_vec(vec![1, 2, 4]);
    let list2 = ListNode::from_vec(vec![1, 3, 4]);
    let result = merge_two_lists(list1, list2);
    assert_eq!(result.unwrap().to_vec(), vec![1, 1, 2, 3, 4, 4]);
  }

  #[test]
  fn merge_two_lists_example_2() {
    let list1 = ListNode::from_vec(vec![]);
    let list2 = ListNode::from_vec(vec![]);
    let result = merge_two_lists(list1, list2);
    assert_eq!(result, None);
  }

  #[test]
  fn merge_two_lists_example_3() {
    let list1 = ListNode::from_vec(vec![]);
    let list2 = ListNode::from_vec(vec![0]);
    let result = merge_two_lists(list1, list2);
    assert_eq!(result.unwrap().to_vec(), vec![0]);
  }
}
