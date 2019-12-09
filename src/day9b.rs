use super::intcode;

pub fn day9b() {
    intcode::run(&mut intcode::input("data/day9.txt"), &vec![2]);
}