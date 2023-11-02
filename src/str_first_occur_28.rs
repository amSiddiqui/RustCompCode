pub struct Solution {}


#[allow(dead_code)]
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let mut i = 0;
        let hs_v: Vec<char> = haystack.chars().collect();
        let ns_v: Vec<char> = needle.chars().collect();
        let (hl, nl) = (hs_v.len(), ns_v.len());
        if nl > hl {
            return -1;
        }
        while i < hl - nl + 1 {
            let orig = i;
            let mut j = 0;
            while j < nl && hs_v[i] == ns_v[j]  {
                i += 1;
                j += 1;
            }
            if j == nl {
                return orig as i32;
            }
            i = orig;
            i += 1;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_str_found() {
        let hs_1 = "sadbutsad".to_string();
        let nl_1 = "sad".to_string();
        assert_eq!(Solution::str_str(hs_1, nl_1), 0);
    }

    #[test]
    fn test_str_str_not_found() {
        let hs_1 = "leetcode".to_string();
        let nl_1 = "leeto".to_string();
        assert_eq!(Solution::str_str(hs_1, nl_1), -1);
    }
}
