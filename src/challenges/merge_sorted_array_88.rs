pub fn run() {

    let mut num1: Vec<i32> = Vec::from([1,2,3,0,0,0]);
    let mut num2: Vec<i32> = Vec::from([2,5,6]);


    merge(&mut num1 , 3, &mut num2, 3);
}

fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {

    if nums2.len() > 0 {
            
        for i in nums2{
             for (j, val) in nums1.iter_mut().enumerate() {
                 if *val == 0 {
                     *val = *i;
                     break;
                 }
             }
        }
    }

    nums1.sort();

    println!("{:?}", nums1);
}
