use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut parens = Vec::new();
        let mut paren_pair = HashMap::new();
        paren_pair.insert(')', '(');
        paren_pair.insert('}', '{');
        paren_pair.insert(']', '[');

        for ch in s.chars() {
            if ch == '(' || ch == '[' || ch == '{' {
                parens.push(ch);
            } else {
                match parens.pop() {
                    Some(paren) => {
                        if !paren_pair.contains_key(&ch) {
                            return false;
                        }
                        if paren_pair[&ch] != paren {
                            return false;
                        }
                    },
                    None => {
                        return false;
                    }
                }

            }
        }

        parens.len() == 0
    }

    pub fn is_valid_fast(s: String) -> bool {
        let mut parens = Vec::new();

        for ch in s.chars() {
            match ch {
                '{' => parens.push('}'),
                '[' => parens.push(']'),
                '(' => parens.push(')'),
                ')'|']'|'}' if Some(ch) != parens.pop() => return false,
                _ => ()
            }
        }

        parens.is_empty()
    }
}