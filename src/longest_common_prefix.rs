pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result = String::from("");

        let smallest_len = strs.iter().map(|s| s.len()).min().unwrap_or(0);
        let l = strs.len();
        let vec_chars: Vec<Vec<char>> = strs.into_iter().map(|s| s.chars().collect()).collect();
        
        let mut i = 0;
        while i < smallest_len {
            let first_ch = vec_chars[0][i];
            for j in 1..l {
                let ch = vec_chars[j][i];
                if ch != first_ch {
                    return result;
                }
            }
            result.push(first_ch);
            i += 1;
        }

        result
    }
}
