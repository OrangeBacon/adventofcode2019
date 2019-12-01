use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn day1b() {
    let path = Path::new("data/day1.txt");
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

    let x: i32 = s
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .map(|mass| {
            let mut fuel = 0;
            fuel += mass/3 - 2;
            let mut fuel_mass = fuel;

            while fuel_mass/3-2 > 0 {
                fuel_mass = fuel_mass/3-2;
                fuel += fuel_mass;
            }

            return fuel
        }).sum();

    println!("{}", x)
}