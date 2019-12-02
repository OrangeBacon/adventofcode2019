use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn day2a() {
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

    let mut x: Vec<usize> = s.split(",").map(|x| x.parse().unwrap()).collect();
    x[1] = 12;
    x[2] = 2;

    let mut pt = 0usize;
    loop {
        match x[pt] {
            1  => {let loc = x[pt+3]; x[loc] = x[x[pt+1]] + x[x[pt+2]]},
            2  => {let loc = x[pt+3]; x[loc] = x[x[pt+1]] * x[x[pt+2]]},
            99 => break,
            _ => unimplemented!("Unimplemented Opcode reached"),
        }
        pt += 4;
    }

    println!("{}", x[0]);
}