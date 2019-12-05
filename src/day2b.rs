use super::intcode;

pub fn day2b() {
    let x = intcode::input("data/day2.txt");

    for a in 0..=99 {
        for b in 0..=99 {
            let mut code = x.clone();
            code[1] = a;
            code[2] = b;
            let ans = intcode::run(&mut code, &vec![]);
            if ans == 19690720 {
                println!("a: {}, b: {}", a, b);
            }
        }
    }
}