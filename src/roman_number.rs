use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut mapper = HashMap::new();
        mapper.insert('I', 1);
        mapper.insert('V', 5);
        mapper.insert('X', 10);
        mapper.insert('L', 50);
        mapper.insert('C', 100);
        mapper.insert('D', 500);
        mapper.insert('M', 1000);
        
        let chars: Vec<char> = s.chars().collect();
        let l = s.len();
        if l == 0 {
            return 0;
        }
        if l == 1 {
            return mapper[&chars[0]];
        }
        let mut result = 0;
        let mut i = 0;
        
        while i < l - 1 {
            let mut cur_val = mapper[&chars[i]];
            let mut next_val = mapper[&chars[i+1]];
            while i < l - 1 && chars[i] == chars[i+1] {
                cur_val += next_val;
                i += 1;
                next_val = mapper[&chars[i]];
            }
            if i == l - 1 {
                result += cur_val;
                i += 1;
                break;
            }
            next_val = mapper[&chars[i+1]];
            if cur_val < next_val {
                result += next_val - cur_val;
                i += 2;
            } else {
                result += cur_val;
                i += 1;
            }
        }

        if i == l - 1 {
            result += mapper[&chars[i]];
        }

        result
    }

    pub fn roman_to_int_fast(s: String) -> i32 {
        let mut map = [0; 26];
        map[8] = 1;
        map[21] = 5;
        map[23] = 10;
        map[11] = 50;
        map[2] = 100;
        map[3] = 500;
        map[12] = 1000;

        let mut sum = 0;
        let mut s = s.into_bytes().into_iter().map(|b| (b - 65) as usize);
        let mut cur = s.next().unwrap();
        while let Some(next) = s.next() {
            sum += if map[cur] >= map[next] { map[cur] } else { -map[cur] };
            cur = next;
        }
        sum + map[cur]
    }
}