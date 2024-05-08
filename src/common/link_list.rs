#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  pub fn new(val: i32) -> Self {
    ListNode { val, next: None }
  }

  pub fn new_with_next(val: i32, next: Option<Box<ListNode>>) -> Self {
    ListNode { val, next }
  }

  pub fn from_vec(vals: Vec<i32>) -> Option<Box<ListNode>> {
    if vals.is_empty() {
      return None;
    }
    let mut head = Box::new(ListNode::new(vals[0]));
    let mut current = &mut head;
    for i in 1..vals.len() {
      current.next = Some(Box::new(ListNode::new(vals[i])));
      current = current.next.as_mut().unwrap();
    }
    Some(head)
  }

  pub fn add_node(&mut self, node: Option<Box<ListNode>>) {
    self.next = node;
  }

  pub fn to_vec(&self) -> Vec<i32> {
    let mut result = vec![self.val];
    let mut current = &self.next;
    while let Some(node) = current {
      result.push(node.val);
      current = &node.next;
    }
    result
  }
}
