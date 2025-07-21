pub fn solve() {
    let result = maximum_time("??:3?".to_owned());
    println!("{:?}", result);
}
fn maximum_time(time: String) -> String {
    let mut i: i8 = -1;
    time.chars()
        .map(|c| {
            i += 1;
            if c == '?' {
                if i == 0 {
                    let ch = time.chars().nth(1).expect("shii");
                    if ch == '?' {
                        return '2';
                    } else {
                        let n = ch.to_digit(10).expect("nahh");
                        if n > 3 {
                            return '1';
                        } else {
                            return '2';
                        }
                    }
                } else if i == 1 {
                    let ch = time.chars().nth(0).expect("shii");
                    if ch == '2' || ch == '?' {
                        return '3';
                    } else {
                        return '9';
                    }
                } else if i == 3 {
                    return '5';
                } else {
                    return '9';
                }
            }
            return c;
        })
        .collect()
}
