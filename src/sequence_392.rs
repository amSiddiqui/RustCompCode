pub struct Solution {}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s_vec: Vec<char> = s.chars().collect();
        let t_vec: Vec<char> = t.chars().collect();

        let mut k: usize = 0;
        let mut found: usize = 0;
        for i in 0..s_vec.len() {
            for j in k..t_vec.len() {
                if s_vec[i] == t_vec[j] {
                    k = j + 1;
                    found += 1;
                    break;
                }
            }
        }

        found == s_vec.len()
        
    }
}