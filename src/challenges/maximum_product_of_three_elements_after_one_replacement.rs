pub fn run() {
    //let arr = Vec::from([-5, 7, 0]);
    let arr = Vec::from([-4, -2, -1, -3]);
    println!("result: {:?}", max_product(arr));
}

fn max_product(nums: Vec<i32>) -> i64 {
    let mut result: i64 = 1;
    let mut nums = nums.clone();

    for i in nums.iter_mut() {
        if *i < 0 {
            *i = i.abs();
        }
    }

    nums.sort();

    for i in &nums[nums.len() - 2..] {
        result *= *i as i64;
    }
    if result > 0 {
        result *= 10i64.pow(5);
    } else {
        result *= -10i64.pow(5);
    }
    result
}
