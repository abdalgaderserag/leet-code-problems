use std::collections::HashMap;

pub fn solve() {
    let nums = vec![2, 2, 1];

    let result = single_number(nums);
    println!("{:?}", result);
}

fn single_number(nums: Vec<i32>) -> i32 {
    let mut hash = HashMap::new();
    for n in nums {
        *hash.entry(n).or_insert(0) += 1;
    }
    let t: Vec<i32> = hash
        .iter()
        .filter(|&(_, &v)| v == 1)
        .map(|(&k, _)| k)
        .collect();
    t[0]
}
