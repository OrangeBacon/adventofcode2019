use std::collections::HashMap;
use std::cmp;
use super::intcode;

enum State {
    Color,
    Direction,
}

pub fn day11b() {
    let code = intcode::input("data/day11.txt");
    let mut data = intcode::RunData::new(&code);

    let mut locations: HashMap<(i32, i32), bool> = HashMap::new();
    let mut current_location = (0,0);
    let mut direction = 0;

    let mut output_state = State::Color;

    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;

    locations.insert(current_location, true);
    
    loop {
        match intcode::run_yield(&mut data) {
            intcode::Interrupt::Input => {
                data.input(*locations.get(&current_location).unwrap_or(&false) as i64)
            }
            intcode::Interrupt::Output(num) => {
                match output_state {
                    State::Color => {
                        output_state = State::Direction;
                        let insert_val = match num {
                            0 => false,
                            1 => true,
                            _ => unreachable!(),
                        };
                        locations.insert(current_location, insert_val);
                    },
                    State::Direction => {
                        output_state = State::Color;
                        direction = match num {
                            0 => if direction - 1 < 0 {3} else {direction - 1},
                            1 => (direction + 1)%4,
                            _ => unreachable!(),
                        };
                        match direction {
                            0 => current_location.1 += 1,
                            1 => current_location.0 += 1,
                            2 => current_location.1 -= 1,
                            3 => current_location.0 -= 1,
                            _ => unreachable!(),
                        }
                        min_x = cmp::min(min_x, current_location.0);
                        min_y = cmp::min(min_y, current_location.1);
                        max_x = cmp::max(max_x, current_location.0);
                        max_y = cmp::max(max_y, current_location.1);
                    }
                }
            }
            intcode::Interrupt::Halt => {
                break;
            }
        }
    }

    println!("{}:{} -> {}:{} @ {}", min_x, min_y, max_x, max_y, locations.len());

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            if *locations.get(&(x,y)).unwrap_or(&false) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}