use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::cmp::{max, min};

enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Location {
    None,
    First(i32),
}

fn read(line: &str) -> Vec<Direction> {
    line.split(",").map(|item| {
        let num = (&item[1..]).parse::<i32>().unwrap();
        match item.chars().nth(0).unwrap() {
            'U' => Direction::Up(num),
            'D' => Direction::Down(num),
            'L' => Direction::Left(num),
            'R' => Direction::Right(num),
            _ => unimplemented!(),
        }
    }).collect()
}

pub fn day3b() {
    let path = Path::new("data/day3.txt");
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

    let x : Vec<&str> = s.lines().collect();
    let line1 = read(x[0]);
    let line2 = read(x[1]);

    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    let mut current_x = min_x;
    let mut current_y = min_y;
    for item in &line1 {
        match item {
            Direction::Up(count) => {current_y = current_y + *count; max_y = max(current_y, max_y)},
            Direction::Down(count) => {current_y = current_y - *count; min_y = min(current_y, min_y)},
            Direction::Left(count) => {current_x = current_x - *count; min_x = min(current_x, min_x)},
            Direction::Right(count) => {current_x = current_x + *count; max_x = max(current_x, max_x)},
        }
    }

    current_x = 0;
    current_y = 0;
    for item in &line2 {
        match item {
            Direction::Up(count) => {current_y = current_y + *count; max_y = max(current_y, max_y)},
            Direction::Down(count) => {current_y = current_y - *count; min_y = min(current_y, min_y)},
            Direction::Left(count) => {current_x = current_x - *count; min_x = min(current_x, min_x)},
            Direction::Right(count) => {current_x = current_x + *count; max_x = max(current_x, max_x)},
        }
    }

    max_x -= min_x;
    max_y -= min_y;

    let mut map = vec![vec![Location::None; max_y as usize+2];max_x as usize+2];

    current_x = -min_x;
    current_y = -min_y;
    let mut distance = 0;
    for item in &line1 {
        match item {
            Direction::Up(count) => {
                for _ in 0..*count {
                    current_y += 1;
                    distance += 1;
                    match map[current_x as usize][current_y as usize] {
                        Location::First(_) => (),
                        Location::None => map[current_x as usize][current_y as usize] = Location::First(distance),
                    }
                }
            }
            Direction::Down(count) => {
                for _ in 0..*count {
                    current_y -= 1;
                    distance += 1;
                    match map[current_x as usize][current_y as usize] {
                        Location::First(_) => (),
                        Location::None => map[current_x as usize][current_y as usize] = Location::First(distance),
                    }
                }
            }
            Direction::Left(count) => {
                for _ in 0..*count {
                    current_x -= 1;
                    distance += 1;
                    match map[current_x as usize][current_y as usize] {
                        Location::First(_) => (),
                        Location::None => map[current_x as usize][current_y as usize] = Location::First(distance),
                    }
                }
            }
            Direction::Right(count) => {
                for _ in 0..*count {
                    current_x += 1;
                    distance += 1;
                    match map[current_x as usize][current_y as usize] {
                        Location::First(_) => (),
                        Location::None => map[current_x as usize][current_y as usize] = Location::First(distance),
                    }
                }
            }
        }
    }

    current_x = -min_x;
    current_y = -min_y;

    let mut res_distances = vec![];

    distance = 0;
    'out: for item in &line2 {
        match item {
            Direction::Up(count) => {
                for _ in 0..*count {
                    current_y += 1;
                    distance += 1;
                    match map[current_x as usize][current_y as usize] {
                        Location::First(dist) => res_distances.push(dist + distance),
                        _ => (),
                    }
                }
            }
            Direction::Down(count) => {
                for _ in 0..*count {
                    current_y -= 1;
                    distance += 1;
                    match map[current_x as usize][current_y as usize] {
                        Location::First(dist) => res_distances.push(dist + distance),
                        _ => (),
                    }
                }
            }
            Direction::Left(count) => {
                for _ in 0..*count {
                    current_x -= 1;
                    distance += 1;
                    match map[current_x as usize][current_y as usize] {
                        Location::First(dist) => res_distances.push(dist + distance),
                        _ => (),
                    }
                }
            }
            Direction::Right(count) => {
                for _ in 0..*count {
                    current_x += 1;
                    distance += 1;
                    match map[current_x as usize][current_y as usize] {
                        Location::First(dist) => res_distances.push(dist + distance),
                        _ => (),
                    }
                }
            }
        }
    }

    println!("{}", res_distances.iter().min().unwrap());
}