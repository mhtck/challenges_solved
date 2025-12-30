pub fn run() {
    let arr = Vec::from([1, 4, 2, 5]);
    let solve = find_missing_elements(arr);
    println!("result: {:?}", solve);
}

fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut arr = nums.clone();
    arr.sort();
    let first = arr[0];
    let last = arr[arr.len() - 1];

    for i in first + 1..last {
        if arr.iter().find(|&&x| x == i).is_none() {
            result.push(i);
        }
    }
    result
}
