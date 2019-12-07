use std::collections::HashSet;
use super::intcode;

pub fn day7a() {
    let code = intcode::input("data/day7.txt");
    let mut outputs: Vec<i32> = vec![];
    for a in 0..=4 {
        for b in 0..=4 {
            for c in 0..=4 {
                for d in 0..=4 {
                    for e in 0..=4 {
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
                        intcode::run_yield(&mut first, a);
                        let first_val = intcode::run_yield(&mut first, 0).unwrap();

                        let mut second = intcode::RunData {code: code.clone(), ip: 0};
                        intcode::run_yield(&mut second, b);
                        let second_val = intcode::run_yield(&mut second, first_val).unwrap();
                        
                        let mut third = intcode::RunData {code: code.clone(), ip: 0};
                        intcode::run_yield(&mut third, c);
                        let third_val = intcode::run_yield(&mut third, second_val).unwrap();

                        let mut fourth = intcode::RunData {code: code.clone(), ip: 0};
                        intcode::run_yield(&mut fourth, d);
                        let fourth_val = intcode::run_yield(&mut fourth, third_val).unwrap();

                        let mut fifth = intcode::RunData {code: code.clone(), ip: 0};
                        intcode::run_yield(&mut fifth, e);
                        let fifth_val = intcode::run_yield(&mut fifth, fourth_val).unwrap();
                        
                        outputs.push(fifth_val);
                    }
                }
            }
        }
    }

    println!("{:?}", outputs.iter().max().unwrap());
}