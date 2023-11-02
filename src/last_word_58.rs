pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut l = 0;
        let mut found_char = false;
        for ch in s.chars().rev() {
            if !found_char && ch != ' ' {
                found_char = true;
                l += 1;
                continue;
            }
            if found_char && ch == ' ' {
                break;
            }

            if found_char && ch != ' ' {
                l += 1;
            }
        }
        l
    }

    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {

        for n in digits.iter_mut().rev() {
            if *n != 9 {
                *n += 1;
                return digits;
            }
            *n = 0;
        }
        digits.insert(0, 1);
        digits
    }
}


#[cfg(test)]
mod tests {
    use crate::last_word_58::Solution;

    #[test]
    fn test_length_of_last_word() {
        let s = "Hello World".to_string();
        assert_eq!(Solution::length_of_last_word(s), 5);

        let s = "   fly me   to   the moon  ".to_string();
        assert_eq!(Solution::length_of_last_word(s), 4);

        let s = "luffy is still joyboy".to_string();
        assert_eq!(Solution::length_of_last_word(s), 6);
    }

    #[test]
    fn test_plus_one() {
        let s = vec![4,3,2,1];
        assert_eq!(Solution::plus_one(s), vec![4, 3, 2, 2]);

        let s = vec![9];
        assert_eq!(Solution::plus_one(s), vec![1, 0]);
    }
}