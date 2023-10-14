pub struct Solution { }

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let (mut i, mut j) = (0, nums.len());
        while i < j {
            while i < j && nums[j - 1] == val {
                j -= 1;
            }
            if i == j {
                break;
            }
            if nums[i] == val {
                nums.swap(i, j - 1);
                j -= 1;
            }
            i += 1;
        }

        j as i32
    }
}