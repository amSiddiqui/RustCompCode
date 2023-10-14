pub struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut h) = (0, nums.len());

        while l < h {
            let mid = (l + h) / 2;
            if target == nums[mid] {
                return mid as i32;
            }
            if target > nums[mid] {
                l = mid + 1;
            } else {
                h = mid;
            }
        }

        l as i32
    }
}


#[cfg(test)]
mod tests {
    use crate::search_insert_35::Solution;

    #[test]
    fn test_search_insert() {
        let nums = vec![4, 12, 43];
        let target = 15;
        assert_eq!(Solution::search_insert(nums, target), 2);

        let nums = vec![1,3,5,6];
        let target = 5;
        assert_eq!(Solution::search_insert(nums, target), 2);

        let nums = vec![1,3,5,6];
        let target = 2;
        assert_eq!(Solution::search_insert(nums, target), 1);

        let nums = vec![1,3,5,6];
        let target = 7;
        assert_eq!(Solution::search_insert(nums, target), 4);
    }
}