fn calculate_sum(arr: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for i in arr.iter() {
        match sum.checked_add(*i) {
            Some(s) => sum = s,
            None => return None,
        }
    }
    Some(sum)
}

pub fn run() {
    let arr1: [u32; 3] = [1, 2, 3];
    let sum1 = calculate_sum(&arr1);
    println!("array: {:?}\nsum: {:?}\n", arr1, sum1);

    let arr2: [u32; 3] = [1, u32::MAX, 3];
    let sum2 = calculate_sum(&arr2);
    println!("array: {:?}\nsum: {:?}\n", arr2, sum2);
}
