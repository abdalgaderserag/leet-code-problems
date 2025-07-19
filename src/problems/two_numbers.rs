pub fn solve() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    if let Some(result) = two_sum(nums, target) {
        println!("Indices: {:?}", result);
    } else {
        println!("No solution found.");
    }
}

fn two_sum(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
    use std::collections::HashMap;
    let mut map = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = map.get(&complement) {
            return Some((j, i));
        }
        map.insert(num, i);
    }
    None
}
