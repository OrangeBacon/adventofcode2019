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

                        let mut first = intcode::RunData {code: code.clone(), ip: 0};
                        let mut second = intcode::RunData {code: code.clone(), ip: 0};
                        let mut third = intcode::RunData {code: code.clone(), ip: 0};
                        let mut fourth = intcode::RunData {code: code.clone(), ip: 0};
                        let mut fifth = intcode::RunData {code: code.clone(), ip: 0};

                        intcode::run_yield(&mut first, a);
                        let mut first_val = intcode::run_yield(&mut first, 0).unwrap();

                        intcode::run_yield(&mut second, b);
                        let mut second_val = intcode::run_yield(&mut second, first_val).unwrap();
                    
                        intcode::run_yield(&mut third, c);
                        let mut third_val = intcode::run_yield(&mut third, second_val).unwrap();

                        intcode::run_yield(&mut fourth, d);
                        let mut fourth_val = intcode::run_yield(&mut fourth, third_val).unwrap();

                        intcode::run_yield(&mut fifth, e);
                        let mut final_val = intcode::run_yield(&mut fifth, fourth_val).unwrap();

                        loop {
                            first_val = match intcode::run_yield(&mut first, final_val) {
                                Some(a) => a,
                                None => break,
                            };

                            second_val = intcode::run_yield(&mut second, first_val).unwrap();
                            third_val = intcode::run_yield(&mut third, second_val).unwrap();
                            fourth_val = intcode::run_yield(&mut fourth, third_val).unwrap();
                            final_val = intcode::run_yield(&mut fifth, fourth_val).unwrap();
                        }
                        
                        outputs.push(final_val);
                    }
                }
            }
        }
    }

    println!("{:?}", outputs.iter().max().unwrap());
}