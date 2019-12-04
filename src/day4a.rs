fn check(mut password: i32) -> bool {
    let mut adj = false;
    let mut prev = 10;

    for _ in 0..6 {
        let current = password % 10;
        if current > prev {
            return false;
        } else if current == prev {
            adj = true;
        }
        password -= current;
        password /= 10;
        prev = current;
    }

    adj
}

pub fn day4a() {
    let mut count = 0;

    for i in 367479..893698 {
        if check(i) {count += 1}
    }

    println!("{}", count);
}