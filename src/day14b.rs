use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug)]
struct Mapping<'a> {
    quantity: i64,
    from: Vec<(&'a String, i64)>
}

pub fn day14b() {
    let path = Path::new("data/day14.txt");
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

    let mut quantities = HashMap::new();
    let mut makes = HashMap::new();

    let mut fuel = &"".to_string();

    let data: Vec<Vec<String>> = s.lines().map(|x| x.split(" ").map(|a| a.to_string()).map(|a| a.chars().filter(|ch| *ch != ',').collect()).collect()).collect();
    for (idx, line) in data.iter().enumerate() {
        let mut i = line.len() - 1;
        
        let output = &data[idx][i]; i -= 1;
        quantities.insert(output, 0);

        if output == "FUEL" {
            fuel = output;
        }

        let amount = line[i].parse::<i64>().unwrap();
        i -= 2;

        let mut from = vec![];
        while let Some(name) = line.get(i) {
            let num = line[i-1].parse::<i64>().unwrap();
            from.push((name, num));
            if i < 2 {
                break;
            }
            i -= 2;
        }

        makes.insert(output, Mapping {
            quantity: amount,
            from,
        });
    }

    let mut start = 0;
    let mut end = 1000000000000;
    let mut guess;
    loop {
        guess = (end-start)/2+start;
        let used = produce(fuel, guess, &makes, &mut quantities.clone());

        if used == 1000000000000 {
            break;
        }

        if used > 1000000000000 {
            println!("try {}, too high, used {}", guess, used);
            end = guess;
        } else {
            start = guess;
            println!("try {}, too low, used {}", guess, used);
        }
        
    }

    println!("{}", guess);
}

fn produce<'a>(desired: &'a String, mut count: i64, data: &'a HashMap<&String, Mapping>, quantities: &mut HashMap<&'a String, i64>) -> i64 {
    if desired == "ORE" {
        return count;
    }

    if quantities[desired] >= count {
        quantities.insert(desired, quantities[desired]-count);
        return 0;
    }

    if quantities[desired] > 0 {
        count -= quantities[desired];
        quantities.insert(desired, 0);
    }

    let mapping = &data[desired];
    let runs = (count as f64 / mapping.quantity as f64).ceil() as i64;

    let mut ore = 0;
    for a in &mapping.from {
        ore += produce(a.0, a.1 * runs, data, quantities);
    }

    let produced = runs * mapping.quantity;
    let excess = produced - count;
    quantities.insert(desired, quantities[desired]+excess);

    ore
}