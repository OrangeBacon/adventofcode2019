use super::intcode;

pub fn day2a() {
    let mut x = intcode::input("data/day2.txt");
    x[1] = 12;
    x[2] = 2;

    intcode::run(&mut x, &vec![]);

    println!("{}", x[0]);
}