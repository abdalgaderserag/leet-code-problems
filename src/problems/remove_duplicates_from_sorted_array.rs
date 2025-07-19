pub fn solve() {
    let nums = vec![1, 1, 2];

    let result = remove_duplicates(nums);
    println!("{:?}", result);
}
fn remove_duplicates(mut nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut last = nums[0];
    let mut s: Vec<usize> = Vec::new();

    for i in 1..nums.len() {
        if nums[i] == last {
            s.push(i);
        } else {
            last = nums[i];
        }
    }

    for &index in s.iter().rev() {
        nums.remove(index);
    }

    nums.len() as i32
}
