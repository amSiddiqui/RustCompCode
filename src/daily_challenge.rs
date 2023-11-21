use std::collections::{BTreeMap, HashMap};

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

    pub fn garbage_collection(garbage: Vec<String>, mut travel: Vec<i32>) -> i32 {
        if garbage.is_empty() {
            return 0
        }
        travel.insert(0, 0);

        for i in 1..travel.len() {
            travel[i] += travel[i - 1];
        }

        let mut garbage_pick_time = 0;
        let (mut m_i, mut p_i, mut g_i) = (0, 0, 0);
        for (i, garbage_str) in garbage.into_iter().enumerate() {
            if garbage_str.contains('M') {
                m_i = i;
            }
            if garbage_str.contains('P') {
                p_i = i;
            }
            if garbage_str.contains('G') {
                g_i = i;
            }
            garbage_pick_time += garbage_str.len();
        }

        garbage_pick_time as i32 + travel[m_i] + travel[p_i] + travel[g_i]
    }

    fn rev_diff(num: i32) -> i32 {
        let rev_str: String = num.to_string().chars().rev().collect();
        num - rev_str.parse().unwrap_or(0)
    }

    fn rev_diff_num(num: i32) -> i32 {
        let mut rev = 0;
        let mut n = num;
        while n > 0 {
            rev = rev * 10 + n % 10;
            n /= 10;
        }
        num - rev
    }

    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        let rev_nums: Vec<i32> = nums.into_iter().map(Solution::rev_diff_num).collect();
        let mut freq_count = HashMap::new();
        for n in rev_nums {
            *freq_count.entry(n).or_insert(0) += 1;
        }
        let mut result: i64 = 0;

        for val in freq_count.values() {
            result += val * (val - 1) / 2;
        }

        (result % 1000000007) as i32
    }
}


#[cfg(test)]
mod tests {
    use crate::daily_challenge::Solution;
    use std::fs;
    use std::time::Instant;
    use rand::Rng;

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

    #[test]
    fn test_garbage_collection() {
        let arr = vec!["G".to_string(), "P".to_string(), "GP".to_string(), "GG".to_string()];
        let travels = vec![2,4,3];

        let result = Solution::garbage_collection(arr, travels);
        assert_eq!(result, 21);

        let arr:Vec<String> = vec!["MMM", "PGM", "GP"].into_iter().map(String::from).collect();
        let travels = vec![3, 10];
        let result = Solution::garbage_collection(arr, travels);

        assert_eq!(result, 37);
    }

    #[test]
    fn test_rev_diff() {
        let mut rng = rand::thread_rng();
        let n = 100000;
        let arr:Vec<i32> = (0..n).map(|_| rng.gen_range(0..=999999)).collect();
        let arr2 = arr.clone();
        let start = Instant::now();
        for a in arr {
            Solution::rev_diff_num(a);
        }
        let duration = start.elapsed();
        println!("Rev diff num time elapsed {:?}", duration);
        let start = Instant::now();
        for a in arr2 {
            Solution::rev_diff(a);
        }
        let duration = start.elapsed();
        println!("Rev diff String time elapsed {:?}", duration);
    }

    #[test]
    fn test_count_nice_pairs() {
        let arr = vec![42, 11, 1, 97];
        assert_eq!(Solution::count_nice_pairs(arr), 2);

        let arr = vec![13, 10, 35, 24, 76];
        assert_eq!(Solution::count_nice_pairs(arr), 4);
    }

    #[test]
    fn test_count_nice_pairs_large() {
        let input_file = "data/daily_challenge/count_nice_pairs";
        let file_content = fs::read_to_string(input_file);
        match file_content {
            Ok(file_content) => {
                let arr: Vec<i32> = file_content.split(',')
                    .map(|a| a.parse()
                        .unwrap_or(0))
                    .collect();
                let result = Solution::count_nice_pairs(arr);
                assert_eq!(result, 999949972);
            },
            Err(e) => {
                println!("Cannot read file {}", e);
            }
        }
    }
}