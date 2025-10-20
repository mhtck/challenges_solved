mod challenges;

use std::io;

fn main() {
    println!("=== LeetCode Problem Runner ===");

    let challenges = challenges::list_available_challenges();
    println!("Available challenges:");
    for (id, name) in &challenges {
        println!("{}: {}", id, name);
    }

    println!("\nEnter the problem ID to run:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim().parse::<u32>() {
        Ok(id) => challenges::run_problem_by_id(id),
        Err(_) => println!("Invalid input. Please enter a numeric ID."),
    }
}
