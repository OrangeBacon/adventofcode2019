use super::intcode;

pub fn day5b() {
    intcode::run(&mut intcode::input("data/day5.txt"), &vec![5]);
}