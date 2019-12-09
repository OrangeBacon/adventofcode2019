use super::intcode;

pub fn day9a() {
    intcode::run(&mut intcode::input("data/day9.txt"), &vec![1]);
}