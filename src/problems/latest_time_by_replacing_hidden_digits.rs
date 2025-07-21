pub fn solve() {
    let result = maximum_time("2?:?0".to_owned());
    println!("{:?}", result);
}
fn maximum_time(time: String) -> String {
    let mut i = 0;
    let mut skip = false;
    let mut yo: bool = false;
    time.chars()
        .map(|c| {
            if c == '?' {
                let mut ch: char;
                if i == 0 {
                    if let Some(second_char) = time.chars().nth(1) {
                        if second_char == '?' {
                            return '2';
                        }
                        let second = second_char.to_digit(10).expect("nah");
                        if second == 9 {
                            return '0';
                        } else if second > 4 {
                            return '1';
                        }
                        return '2';
                    } else {
                        ch = '2';
                    }
                } else if i == 1 {
                    if let Some(char) = time.chars().nth(0) {
                        let second = char.to_digit(10).expect("nah");
                        if second == 2 {
                            return '4';
                        }
                        return '9';
                    } else {
                        ch = '2';
                    }
                } else if i == 1 && yo {
                    ch = '3';
                } else if i == 3 {
                    ch = '5';
                } else {
                    ch = '9';
                }
                i += 1;
                return ch;
            } else {
                if c == '2' && i == 0 {
                    yo = true;
                }
                i += 1;
                return c;
            }
        })
        .collect()
}
