use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn input(path: &str) -> Vec<i32> {
    let path = Path::new(path);
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

    s.split(",").map(|x| {
        match x.as_bytes()[0] {
            b'-' => -(x[1..].parse::<i32>().unwrap()),
            _ => x.parse().unwrap(),
        }
        
    }).collect()
}
