use std::collections::HashSet;
use super::intcode;

pub fn day7b() {
    let code = intcode::input("data/day7.txt");
    let mut outputs: Vec<i32> = vec![];
    for a in 5..=9 {
        for b in 5..=9 {
            for c in 5..=9 {
                for d in 5..=9 {
                    for e in 5..=9 {
                        let mut inputs = HashSet::new();
                        inputs.insert(a);
                        inputs.insert(b);
                        inputs.insert(c);
                        inputs.insert(d);
                        inputs.insert(e);
                        if inputs.len() != 5 {
                            continue;
                        }

                        let mut first = intcode::RunData::new(&mut code.clone());
                        let mut second = intcode::RunData::new(&mut code.clone());
                        let mut third = intcode::RunData::new(&mut code.clone());
                        let mut fourth = intcode::RunData::new(&mut code.clone());
                        let mut fifth = intcode::RunData::new(&mut code.clone());

                        first.input_vec(&[a, 0]);
                        let mut first_val = intcode::run_yield(&mut first).as_output();

                        second.input_vec(&[b, first_val]);
                        let mut second_val = intcode::run_yield(&mut second).as_output();
                    
                        third.input_vec(&[c, second_val]);
                        let mut third_val = intcode::run_yield(&mut third).as_output();

                        fourth.input_vec(&[d, third_val]);
                        let mut fourth_val = intcode::run_yield(&mut fourth).as_output();

                        fifth.input_vec(&[e, fourth_val]);
                        let mut final_val = intcode::run_yield(&mut fifth).as_output();

                        loop {
                            first.input(final_val);
                            first_val = match intcode::run_yield(&mut first) {
                                intcode::Interrupt::Output(a) => a,
                                _ => break,
                            };

                            second.input(first_val);
                            second_val = intcode::run_yield(&mut second).as_output();

                            third.input(second_val);
                            third_val = intcode::run_yield(&mut third).as_output();

                            fourth.input(third_val);
                            fourth_val = intcode::run_yield(&mut fourth).as_output();

                            fifth.input(fourth_val);
                            final_val = intcode::run_yield(&mut fifth).as_output();
                        }
                        
                        outputs.push(final_val);
                    }
                }
            }
        }
    }

    println!("{:?}", outputs.iter().max().unwrap());
}