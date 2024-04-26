pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() as i32 - 1;
    let mut mid = 0;

    while left <= right {
        mid = left + (right - left) / 2;
        if nums[mid as usize] == target {
            return mid;
        } else if nums[mid as usize] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    return if target > nums[mid as usize] {
        mid + 1
    } else {
        mid
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn example_1_search_insert() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        let expected = 2;
        assert_eq!(super::search_insert(nums, target), expected);
    }

    #[test]
    fn example_2_search_insert() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;
        let expected = 1;
        assert_eq!(super::search_insert(nums, target), expected);
    }

    #[test]
    fn example_3_search_insert() {
        let nums = vec![1, 3, 5, 6];
        let target = 7;
        let expected = 4;
        assert_eq!(super::search_insert(nums, target), expected);
    }

    #[test]
    fn example_4_search_insert() {
        let nums = vec![1, 3, 5, 6];
        let target = 0;
        let expected = 0;
        assert_eq!(super::search_insert(nums, target), expected);
    }

    #[test]
    fn example_5_search_insert() {
        let nums = vec![1, 3];
        let target = 2;
        let expected = 1;
        assert_eq!(super::search_insert(nums, target), expected);
    }
}
