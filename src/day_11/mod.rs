use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut ret_val = Vec::<i32>::new();

    let mut index_map: HashMap<i32, usize> = HashMap::new();
    let mut delta_vec: Vec<i32> = Vec::new();
    for i in 0..nums.len() {
        index_map.insert(nums[i], i);
    }
    for i in 0..nums.len() {
        delta_vec.push(target - nums[i]);
    }
    for i in 0..nums.len() {
        let delta = delta_vec[i];
        if index_map.contains_key(&delta) {
            let index = index_map.get(&delta).unwrap();
            if *index != i {
                ret_val.push(i as i32);
                ret_val.push(*index as i32);
                index_map.remove(&delta);
                index_map.remove(&nums[i]);
            }
        }
    }

    ret_val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let expected = vec![0, 1];
        assert_eq!(two_sum(nums, target), expected);
    }

    #[test]
    fn example_2_two_sum() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let expected = vec![1, 2];
        assert_eq!(two_sum(nums, target), expected);
    }

    #[test]
    fn example_3_two_sum() {
        let nums = vec![3, 3];
        let target = 6;
        let expected = vec![0, 1];
        assert_eq!(two_sum(nums, target), expected);
    }
}
