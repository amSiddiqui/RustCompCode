use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut l = nums.len();
        if l < 3 {
            return result;
        }
        let mut nums_m = nums.clone();
        let nums_se: HashSet<_> = nums.into_iter().collect();
        nums_m.sort();

        match nums_m.iter().position(|&x| x == 0) {
            Some(index) => {
                let zero_count = nums_m[index..l].iter().take_while(|&num| *num == 0).count();
                if zero_count >= 3 {
                    result.push(vec![0, 0, 0]);
                }
                for _ in 0..zero_count - 1 {
                    nums_m.remove(index);
                }
            },
            None => {}
        }

        l = nums_m.len();

        if l < 3 || nums_m[0] >= 0 || nums_m[l - 1] <= 0 {
            return result;
        }

        let mut compared_pairs: HashSet<(i32, i32)> = HashSet::new();

        match nums_m.iter().position(|&x| x > 0) {
            Some(index) => {
                for i in 0..index - 1 {
                    for j in i+1..index {
                        let (a, b) = (nums_m[i], nums_m[j]);
                        if compared_pairs.contains(&(a, b)) {
                            continue;
                        }
                        compared_pairs.insert((a, b));
                        let sum = (a + b) * -1;
                        if nums_se.contains(&sum) {
                            result.push(vec![a, b, sum]);
                        }
                    }
                }
                for i in index..l - 1 {
                    for j in i+1..l {
                        let (a, b) = (nums_m[i], nums_m[j]);
                        if compared_pairs.contains(&(a, b)) {
                            continue;
                        }
                        compared_pairs.insert((a, b));
                        let sum = (a + b) * -1;
                        if nums_se.contains(&sum) {
                            result.push(vec![a, b, sum]);
                        }
                    }
                }
            }
            None => {}
        }
        result
    }


    pub fn three_sum_fast(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        nums.sort();

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i-1] {
                continue;
            }

            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum < 0 {
                    left += 1;
                } else if sum > 0 {
                    right -= 1;
                } else {
                    result.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    while nums[left] == nums[left - 1] && left < right {
                        left += 1;
                    }
                }
            }
        }

        result
    }
}
