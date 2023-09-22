use std::collections::HashMap;

pub struct Solution {}


impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut indices = HashMap::new();
        for (i, v) in nums.iter().enumerate() {
            indices.insert(v, i);
        }

        for (i, val) in nums.iter().enumerate() {
            let diff = target - val;
            if indices.contains_key(&diff) && indices[&diff] != i {
                return vec![i as i32, indices[&diff] as i32];
            }
        }
        vec![]
    }
}