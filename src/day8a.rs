use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn day8a() {
    let path = Path::new("data/day8.txt");
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

    let x: Vec<u32> = s.chars().map(|x| x.to_digit(10).unwrap()).collect();
    let mut iter = x.iter();

    let mut layers = vec![];

    let width = 25;
    let height = 6;

    while iter.len() > 0 {
        let mut layer = vec![];
        for _ in 0..height {
            let mut row = vec![];
            for _ in 0..width {
                row.push(iter.next().unwrap());
            }
            layer.push(row);
        }
        layers.push(layer);
    }

    let mut zeros = vec![];
    for layer in &layers {
        let mut sum = 0;
        for row in layer {
            sum += row.iter().filter(|x| ***x == 0).count();
        }
        zeros.push(sum);
    }

    let layer_idx = zeros.iter().enumerate()
        .min_by(|&(_, a), &(_, b)|a.cmp(b)).unwrap().0;
    let layer = &layers[layer_idx];

    let ones = layer.iter().flatten().filter(|x| ***x == 1).count();
    let twos = layer.iter().flatten().filter(|x| ***x == 2).count();

    println!("{}", ones * twos);
}