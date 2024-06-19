use std::collections::HashMap;

fn is_cyclic(course: i32, graph: &HashMap<i32, Vec<i32>>, visited: &mut Vec<bool>, rec_stack: &mut Vec<bool>) -> bool {
  visited[course as usize] = true;
  rec_stack[course as usize] = true;

  if let Some(pre) = graph.get(&course) {
    for i in 0..pre.len() {
      if rec_stack[pre[i] as usize] {
        return true;
      }

      if !visited[pre[i] as usize] && is_cyclic(pre[i], graph, visited, rec_stack) {
        return true;
      }
    }
  }

  rec_stack[course as usize] = false;
  false
}

pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
  let mut graph = HashMap::<i32, Vec<i32>>::new();
  let mut visited = vec![false; num_courses as usize];
  let mut rec_stack = vec![false; num_courses as usize];

  for (_, pre) in prerequisites.iter().enumerate() {
    let course = pre[0];
    let pre_req = pre[1];
    let entry = graph.entry(course).or_insert(vec![]);
    entry.push(pre_req);
  }

  for i in 0..num_courses {
    if !visited[i as usize] {
      if is_cyclic(i, &graph, &mut visited, &mut rec_stack) {
        return false;
      }
    }
  }

  true
}

#[cfg(test)]
mod test {
  #[test]
  fn example_1() {
    let num_courses = 2;
    let prerequisites = vec![vec![1, 0]];
    let result = super::can_finish(num_courses, prerequisites);
    assert_eq!(result, true);
  }

  #[test]
  fn example_2() {
    let num_courses = 2;
    let prerequisites = vec![vec![1, 0], vec![0, 1]];
    let result = super::can_finish(num_courses, prerequisites);
    assert_eq!(result, false);
  }
}
