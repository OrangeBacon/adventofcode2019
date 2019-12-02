use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn run_intcode(x: &mut Vec<usize>, a: usize, b: usize) -> usize {
    x[1] = a;
    x[2] = b;

    let mut ip = 0usize;
    loop {
        match x[ip] {
            1  => {let loc = x[ip+3]; x[loc] = x[x[ip+1]] + x[x[ip+2]]},
            2  => {let loc = x[ip+3]; x[loc] = x[x[ip+1]] * x[x[ip+2]]},
            99 => break,
            _ => unimplemented!("Unimplemented Opcode reached"),
        }
        ip += 4;
    }

    return x[0];
}

pub fn day2b() {
    let path = Path::new("data/day2.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read {}: {}", display, why.description()),
        Ok(_) => {}
    }

    let x: Vec<usize> = s.split(",").map(|x| x.parse().unwrap()).collect();

    for a in 0..=99 {
        for b in 0..=99 {
            let ans = run_intcode(&mut x.clone(), a, b);
            if ans == 19690720 {
                println!("a: {}, b: {}", a, b);
            }
        }
    }
}