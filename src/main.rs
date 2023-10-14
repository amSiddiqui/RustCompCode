mod longest_common_prefix;
mod roman_number;
mod sequence_392;
mod sum3_15;
mod two_sums;
mod valid_paren_20;
mod merge_list_21;
mod duplicates_21;
mod remove_element_27;
mod str_first_occur_28;
mod search_insert_35;
mod last_word_58;
mod medium_questions;
mod easy_questions;
mod linked_list;

fn main() {
    two_sums::Solution::two_sum(vec![2, 7, 11, 15], 9);
    roman_number::Solution::roman_to_int(String::from("MDCXCV"));
    roman_number::Solution::roman_to_int_fast(String::from("MDCXCV"));

    let strs = vec!["val1".to_string(), "val2".to_string(), "va2".to_string()];
    longest_common_prefix::Solution::longest_common_prefix(strs);

    println!(
        "{}",
        sequence_392::Solution::is_subsequence("abc".to_string(), "auobohicpo".to_string())
    );

    sum3_15::Solution::three_sum(vec![-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4, 4, 6, 6]);

    sum3_15::Solution::three_sum_fast(vec![-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4, 4, 6, 6]);

    valid_paren_20::Solution::is_valid(String::from("()[]{}"));
    valid_paren_20::Solution::is_valid_fast(String::from("()[]{}"));


    let list1 = Box::new(merge_list_21::ListNode {
        val: 20,
        next: Some(Box::new(merge_list_21::ListNode {
            val: 32,
            next: Some(Box::new(merge_list_21::ListNode::new(54)))
        }))
    });

    let list2 = Box::new(merge_list_21::ListNode {
        val: 19,
        next: Some(Box::new(merge_list_21::ListNode {
            val: 31,
            next: Some(Box::new(merge_list_21::ListNode {
                val: 32,
                next: Some(Box::new(merge_list_21::ListNode::new(87)))
            }))
        }))
    });

    let result = merge_list_21::Solution::merge_two_lists(Some(list1), Some(list2));

    merge_list_21::Solution::print_ll(result);

    let mut arr = vec![1, 2];
    let result_21 = duplicates_21::Solution::remove_duplicates(&mut arr);
    println!("{:?} :: {}", arr, result_21);

    arr = vec![3,2,2,3];
    let result_27 = remove_element_27::Solution::remove_element(&mut arr, 3);
    println!("{:?} :: {}", arr, result_27);

}
