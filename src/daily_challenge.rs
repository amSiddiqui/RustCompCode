use std::collections::BTreeMap;

#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable_by(|a, b| b.cmp(a));
        if nums.is_empty() {
            return 0;
        }
        let mut maximum = 1;
        for target in 0..nums.len() - 1 {
            if nums.len() - target < maximum {
                break;
            }
            let mut values = k;
            let mut count = 1;
            for i in target + 1..nums.len() {
                if nums[target] - nums[i] > values {
                    break;
                }
                values -= nums[target] - nums[i];
                count += 1;
            }
            if maximum < count {
                maximum = count;
            }
        }
        maximum as i32
    }

    pub fn fast_max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;

        nums.sort_unstable();

        let mut start = 0_usize;
        let mut required = 0;
        let mut result = 0;
        let mut prev = 0;

        for (i, &num) in nums.iter().enumerate() {
            required += (num - prev) * (i - start) as i32;

            while required > k {
                required -= num - nums[start];
                start += 1;
            }

            result = result.max(i + 1 - start);

            prev = num;
        }

        result as _
    }

    pub fn reduction_operations(nums: Vec<i32>) -> i32 {
        let mut freq_map: BTreeMap<i32, i32> = BTreeMap::new();
        for n in nums {
            *freq_map.entry(n).or_insert(0) += 1;
        }
        if let Some((&smallest_key, _)) = freq_map.iter().next() {
            freq_map.remove(&smallest_key);
            let mut acc = 0;
            for (i, (_, val)) in freq_map.iter().enumerate() {
                acc += ((i as i32) + 1) * val;
            }
            acc
        } else {
            0
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::daily_challenge::Solution;
    use std::fs;

    #[test]
    fn test_max_frequency() {
        let arr = vec![1, 2, 4];
        let result = Solution::max_frequency(arr, 5);
        assert_eq!(result, 3);

        let arr = vec![1, 4, 8, 13];
        let result = Solution::max_frequency(arr, 5);
        assert_eq!(result, 2);

        let input_file = "data/daily_challenge/max_frequency_input";

        let file_content  = fs::read_to_string(input_file);
        match file_content {
            Ok(content) => {
                let mut split = content.split(';');
                let arr_str = split.next().unwrap_or("");
                let k_str = split.next().unwrap_or("0");
                let k = k_str.parse::<i32>().unwrap();
                let arr: Result<Vec<i32>, _> = arr_str.split(',')
                                                    .map(|s| s.parse::<i32>())
                                                    .collect();
                match arr {
                    Ok(arr) => {
                        let result = Solution::max_frequency(arr, k);
                        assert_eq!(result, 100000);
                    },
                    Err(e) => {
                        println!("Error parsing the array {}", e);
                    }
                }
            },
            Err(e) => {
                println!("Failed to load content {}", e);
            }
        }
    }

    #[test]
    fn test_reduction_operations() {
        let arr = vec![5, 1, 3];
        let result = Solution::reduction_operations(arr);
        assert_eq!(result, 3);

        assert_eq!(Solution::reduction_operations(vec![1, 1, 1]), 0);
        assert_eq!(Solution::reduction_operations(vec![1, 1, 2, 2, 3]), 4);
    }
}