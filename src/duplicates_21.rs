pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let (mut i, mut j) = (0, 0);
        while j < nums.len() {
            if nums[i] != nums[j] {
                i += 1;
                if i != j {
                    nums.swap(i, j);
                }
            }
            j += 1;
        }
        (i + 1) as i32
    }
}