use std::collections::HashMap;
use std::cmp;
use super::intcode;

pub fn day13b() {
    let mut code = intcode::input("data/day13.txt");
    code[0] = 2;
    let mut data = intcode::RunData::new(&code);

    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut locations = HashMap::new();
    let mut score = 0;

    let mut ball_x = 0;
    let mut paddle_x = 0;

    loop {
        match intcode::run_yield(&mut data) {
            intcode::Interrupt::Halt => break,
            intcode::Interrupt::Output(x) => {
                let y   = intcode::run_yield(&mut data).as_output();
                let num = intcode::run_yield(&mut data).as_output();

                if x == -1 && y == 0 {
                    score = num;
                } else {
                    locations.insert((x,y), num);
                    min_x = cmp::min(min_x, x);
                    min_y = cmp::min(min_y, y);
                    max_x = cmp::max(max_x, x);
                    max_y = cmp::max(max_y, y);

                    match num {
                        3 => paddle_x = x,
                        4 => ball_x = x,
                        _ => (),
                    }
                }
            },
            intcode::Interrupt::Input => {
                data.input((ball_x-paddle_x).min(1).max(-1));
            },
        }
    }
    println!("{}", score);
}