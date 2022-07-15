fn calculate_sum(arr: &[u32]) -> Option<u32> {
    let mut sum: Option<u32> = Some(0);
    for i in arr.iter() {
        sum = match sum {
            Some(s) => s.checked_add(*i),
            None => None,
        };
    }
    sum
}

pub fn run() {
    let arr1: [u32; 3] = [1, 2, 3];
    let sum1 = calculate_sum(&arr1);
    println!("array: {:?}\nsum: {:?}\n", arr1, sum1);

    let arr2: [u32; 3] = [1, 2, u32::MAX];
    let sum2 = calculate_sum(&arr2);
    println!("array: {:?}\nsum: {:?}\n", arr2, sum2);
}
