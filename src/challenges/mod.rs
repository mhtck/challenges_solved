pub mod merge_sorted_array_88;

use std::collections::HashMap;

pub fn run_problem_by_id(id: u32) {
    match id {
        88 => merge_sorted_array_88::run(),
        _ => println!("Problem ID {} is not implemented yet.", id),
    }
}

pub fn list_available_challenges() -> HashMap<u32, &'static str> {
    let mut map = HashMap::new();
    map.insert(88, "Merge Sorted Array");
    map
}
