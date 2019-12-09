use std::collections::HashSet;
use super::intcode;

pub fn day7a() {
    let code = intcode::input("data/day7.txt");
    let mut outputs: Vec<i64> = vec![];
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
                        let mut first = intcode::RunData::new(&mut code.clone());
                        first.input_vec(&[a, 0]);
                        intcode::run_yield(&mut first);
                        let first_val = intcode::run_yield(&mut first).as_output();

                        let mut second = intcode::RunData::new(&mut code.clone());
                        second.input_vec(&[b, first_val]);
                        intcode::run_yield(&mut second);
                        let second_val = intcode::run_yield(&mut second).as_output();
                        
                        let mut third = intcode::RunData::new(&mut code.clone());
                        third.input_vec(&[c, second_val]);
                        intcode::run_yield(&mut third);
                        let third_val = intcode::run_yield(&mut third).as_output();

                        let mut fourth = intcode::RunData::new(&mut code.clone());
                        fourth.input_vec(&[d, third_val]);
                        intcode::run_yield(&mut fourth);
                        let fourth_val = intcode::run_yield(&mut fourth).as_output();

                        let mut fifth = intcode::RunData::new(&mut code.clone());
                        fifth.input_vec(&[e, fourth_val]);
                        intcode::run_yield(&mut fifth);
                        let fifth_val = intcode::run_yield(&mut fifth).as_output();
                        
                        outputs.push(fifth_val);
                    }
                }
            }
        }
    }

    println!("{:?}", outputs.iter().max().unwrap());
}