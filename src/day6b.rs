use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use indexmap::IndexMap;
use std::collections::HashSet;

pub fn day6b() {
    let path = Path::new("data/day6.txt");
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

    let mut nodes: IndexMap<&str, usize> = IndexMap::new();
    let mut edges: Vec<(usize, usize)> = vec![];
    for line in s.lines() {
        let parts: Vec<&str> = line.split(")").collect();
        if !nodes.contains_key(parts[0]) {
            nodes.insert(parts[0], 0);
        }
        let index1 = nodes.get_full(parts[0]).unwrap().0;
        if !nodes.contains_key(parts[1]) {
            nodes.insert(parts[1], 0);
        }
        let index2 = nodes.get_full(parts[1]).unwrap().0;
        edges.push((index1, index2));
    }

    let get_path = |start: usize, end: usize| {
        let mut current = start;
        let mut ret: HashSet<usize> = HashSet::new();
        ret.insert(current);
        while current != end {
            for edge in &edges {
                if edge.1 == current {
                    current = edge.0;
                    break;
                }
            }
            ret.insert(current);
        }
        ret
    };

    let you_path = get_path(nodes.get_full("YOU").unwrap().0, nodes.get_full("COM").unwrap().0);
    let san_path = get_path(nodes.get_full("SAN").unwrap().0, nodes.get_full("COM").unwrap().0);

    println!("{}", you_path.symmetric_difference(&san_path).count() - 2);
}