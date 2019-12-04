fn check(mut password: i32) -> bool {
    let mut adj = false;
    let mut prev = 10;

    let mut prev_match_len = 1;
    let mut has_two = false;

    for _ in 0..6 {
        let current = password % 10;
        if current > prev {
            return false;
        }
        if current == prev {
            adj = true;
            prev_match_len += 1;
        } else {
            if prev_match_len == 2 {
                has_two = true;
            }
            prev_match_len = 1;
        }
        password -= current;
        password /= 10;
        prev = current;
    }

    adj && (has_two || prev_match_len == 2)
}

pub fn day4b() {
    let mut count = 0;

    for i in 367479..893698 {
        if check(i) {count += 1}
    }

    println!("{}", count);
}