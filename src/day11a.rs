use std::collections::HashMap;
use super::intcode;

enum State {
    Color,
    Direction,
}

pub fn day11a() {
    let code = intcode::input("data/day11.txt");
    let mut data = intcode::RunData::new(&code);

    let mut locations: HashMap<(i32, i32), bool> = HashMap::new();
    let mut current_location = (0,0);
    let mut direction = 0;

    let mut output_state = State::Color;
    
    loop {
        match intcode::run_yield(&mut data) {
            intcode::Interrupt::Input => {
                data.input(*locations.get(&current_location).unwrap_or(&false) as i64)
            }
            intcode::Interrupt::Output(num) => {
                match output_state {
                    State::Color => {
                        output_state = State::Direction;
                        locations.insert(current_location, match num {
                            0 => false,
                            1 => true,
                            _ => unreachable!(),
                        });
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
                    }
                }
            }
            intcode::Interrupt::Halt => {
                break;
            }
        }
    }

    println!("{}", locations.len());
}