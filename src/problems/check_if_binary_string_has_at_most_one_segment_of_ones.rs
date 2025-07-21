pub fn solve() {
    let result = check_ones_segment("110".to_string());
    println!("{:?}", result);
}

fn check_ones_segment(s: String) -> bool {
    let mut pointer = 0;
    let mut i = 0;
    for c in s.chars() {
        if c == '1' {
            if i != 0 && pointer != i - 1 {
                return false;
            }
            pointer = i;
        }
        i += 1;
    }
    return true;
}
