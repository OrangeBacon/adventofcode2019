use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use indexmap::IndexMap;

pub fn day6a() {
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

    let root = nodes.get_full("COM").unwrap().0;
    let mut to_scan = vec![root];
    while to_scan.len() > 0 {
        let node_index = to_scan.pop().unwrap();
        let node_value = *nodes.get_index(node_index).unwrap().1;
        for edge in &edges {
            if edge.0 == node_index {
                to_scan.push(edge.1);
                let x = nodes.get_index_mut(edge.1).unwrap().1;
                *x = node_value + 1;
            }
        }
    }

    let mut count = 0;
    for node in &nodes {
        count += node.1
    }

    println!("{}", count);
}