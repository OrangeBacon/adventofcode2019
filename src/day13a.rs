use std::collections::HashMap;
use super::intcode;

pub fn day13a() {
    let code = intcode::input("data/day13.txt");
    let mut data = intcode::RunData::new(&code);

    let mut locations = HashMap::new();

    loop {
        match intcode::run_yield(&mut data) {
            intcode::Interrupt::Halt => break,
            intcode::Interrupt::Output(x) => {
                let y   = intcode::run_yield(&mut data).as_output();
                let num = intcode::run_yield(&mut data).as_output();

                locations.insert((x,y), num);
            },
            intcode::Interrupt::Input => unreachable!(),
        }
    }

    let a = locations.iter().filter(|&(_,&num)| num == 2).count();
    println!("{}", a);
}