pub struct Solution {}


#[allow(dead_code)]
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let mut prod = Solution::find_prod(n, 2);
        for k in 3..n {
            let temp = Solution::find_prod(n, k);
            if temp < prod {
                return prod;
            }
            prod = temp;
        }
        prod
    }

    fn find_prod(n: i32, k: i32) -> i32 {
        let v: i32 = n / k;
        let remainder = n % k;
        v.pow((k - remainder) as u32) * (v + 1).pow((remainder) as u32)
    }

    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let mut best_diff = i32::MAX;
        for i in 0..nums.len() {
            let (mut j, mut k) = (i + 1, nums.len() - 1);
            let t = target - nums[i];
            while j < k {
                let diff = nums[k] + nums[j] - t;
                if diff.abs() < best_diff.abs() {
                    best_diff = diff;
                }
                match diff.signum() {
                    -1 => j += 1,
                    1 => k -= 1,
                    0 => return target,
                    _ => unreachable!()
                }
            }

        }
        target + best_diff
    }
}


#[cfg(test)]
mod tests {
    use crate::medium_questions::Solution;

    #[test]
    fn test_integer_break() {
        assert_eq!(Solution::integer_break(2), 1);
        assert_eq!(Solution::integer_break(10), 36);
    }

    #[test]
    fn test_three_sum_closes() {
        let nums = vec![4, 0, 5, -5, 3, 3, 0, -4, -5];
        let target = -2;
        assert_eq!(Solution::three_sum_closest(nums, target), -2);
    }
}