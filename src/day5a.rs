use super::intcode;

pub fn day5a() {
    intcode::run(&mut intcode::input("data/day5.txt"), &vec![1]);
}