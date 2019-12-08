use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn day8b() {
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
                row.push(*iter.next().unwrap());
            }
            layer.push(row);
        }
        layers.push(layer);
    }

    let mut image = vec![vec![2; width]; height];
    let mut idx = 0;
    while let Some(layer) = layers.get(idx) {
        idx += 1;

        for (x, row) in layer.iter().enumerate() {
            for (y, colour) in row.iter().enumerate() {
                if image[x][y] == 2 {
                    image[x][y] = *colour;
                }
            }
        }
    }

    println!("{:?}", image);
    // copy to clipboard
    // open matlab
    // img = <paste>
    // imshow(reshape(img, 25, 6)', 'InitialMagnification','fit')
}