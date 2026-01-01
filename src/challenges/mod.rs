pub mod binary_tree_inorder_traversal_94;
pub mod binary_tree_preorder_traversal_144;
pub mod find_missing_elements;
pub mod greatst_english_letter_in_upper_and_lower_case_2309;

//pub mod make_array_elements_equal_to_zero_3354;
pub mod maximum_product_of_three_elements_after_one_replacement;
pub mod merge_sorted_array_88;
pub mod simple_bank_system_2043;
pub mod valid_parentheses_20;

use std::collections::HashMap;

pub fn run_problem_by_id(id: u32) {
    match id {
        88 => merge_sorted_array_88::run(),
        2309 => greatst_english_letter_in_upper_and_lower_case_2309::run(),
        20 => valid_parentheses_20::run(),
        //3354 => make_array_elements_equal_to_zero_3354::run(),
        94 => binary_tree_inorder_traversal_94::run(),
        144 => binary_tree_preorder_traversal_144::run(),
        1 => find_missing_elements::run(),
        2 => maximum_product_of_three_elements_after_one_replacement::run(),
        2043 => simple_bank_system_2043::run(),
        _ => println!("Problem ID {} is not implemented yet.", id),
    }
}

pub fn list_available_challenges() -> HashMap<u32, &'static str> {
    let mut map = HashMap::new();
    map.insert(88, "Merge Sorted Array");
    map.insert(2309, "Greatest English Letter in Upper and Lower Case");
    map.insert(20, "Valid Parentheses");
    map.insert(3354, "Make Array Elements Equal to Zero");
    map.insert(94, "Binary Tree Inorder Traversal");
    map.insert(144, "Binary Tree Preorder Traversal");
    map.insert(1, "Find Missing Elements");
    map.insert(2, "Maximum Product of Three Elements After One Replacement");
    map.insert(2043, "Simple Bank System");

    map
}
