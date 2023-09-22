mod two_sums;
mod roman_number;
mod longest_common_prefix;
mod sequence_392;

fn main() {
    two_sums::Solution::two_sum(vec![2, 7, 11, 15], 9);
    roman_number::Solution::roman_to_int(String::from("MDCXCV"));
    roman_number::Solution::roman_to_int_fast(String::from("MDCXCV"));
    
    let strs = vec!["val1".to_string(), "val2".to_string(), "va2".to_string()];
    longest_common_prefix::Solution::longest_common_prefix(strs);

    println!("{}", sequence_392::Solution::is_subsequence("abc".to_string(), "auobohicpo".to_string()));
}
