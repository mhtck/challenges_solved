pub mod greatst_english_letter_in_upper_and_lower_case_2309;
pub mod merge_sorted_array_88;
pub mod valid_parentheses_20;

use std::collections::HashMap;

pub fn run_problem_by_id(id: u32) {
    match id {
        88 => merge_sorted_array_88::run(),
        2309 => greatst_english_letter_in_upper_and_lower_case_2309::run(),
        20 => valid_parentheses_20::run(),
        _ => println!("Problem ID {} is not implemented yet.", id),
    }
}

pub fn list_available_challenges() -> HashMap<u32, &'static str> {
    let mut map = HashMap::new();
    map.insert(88, "Merge Sorted Array");
    map.insert(2309, "Greatest English Letter in Upper and Lower Case");
    map.insert(20, "Valid Parentheses");
    map
}
