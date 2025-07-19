pub fn solve() {
    let result = is_power_of_three(2069870691);
    println!("{:?}", result);
}

fn is_power_of_three(n: i32) -> bool {
    if n == 0 {
        return false;
    }
    if n == 1 {
        return true;
    }
    if n % 3 != 0 {
        return false;
    }
    let mut i = 1;
    let n: i64 = n.into();
    loop {
        let temp = self::power(i);
        if temp == n {
            return true;
        }
        if temp > n {
            return false;
        }
        i = i + 1;
    }
}

fn power(n: i32) -> i64 {
    if n == 1 {
        return 1;
    }
    return 3 * self::power(n - 1);
}
