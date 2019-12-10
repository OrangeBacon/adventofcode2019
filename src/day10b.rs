use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashSet;
use std::f32;
use num::Integer;

pub fn day10b() {
    let path = Path::new("data/day10.txt");
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

    let mut map: Vec<Vec<bool>> = s.lines().map(|line| {
        line.chars().map(|ch| {
            match ch {
                '.' => false,
                '#' => true,
                _ => unreachable!(),
            }
        }).collect()
    }).collect();

    let mut res = vec![vec![0; map[0].len()]; map.len()];
    for (y, row) in map.iter().enumerate() {
        for (x, loc) in row.iter().enumerate() {
            if !loc {
                continue;
            }
            let mut points = HashSet::new();
            for (line_y, line_row) in map.iter().enumerate() {
                for (line_x, line_loc) in line_row.iter().enumerate() {
                    if *line_loc {
                        line(&mut points, &map, x as f32, y as f32,
                            line_x as f32, line_y as f32);
                    }
                }
            }
            res[y][x] = points.len();
        }
    }

    let out = res
        .iter()
        .enumerate()
        .map(|(y,row)| (y,row.iter().enumerate().max_by(|&(_,a),&(_,b)|a.cmp(b)).unwrap()))
        .max_by(|&(_,(_,a)),&(_,(_,b))|a.cmp(b))
        .unwrap();

    let centre_x = (out.1).0;
    let centre_y = out.0;
    println!("{}:{}", centre_x, centre_y);

    let mut prev_angle = -100.0;
    for _ in 0..200 {
        let mut coords = vec![];
        for (y, row) in map.iter().enumerate() {
            for (x, loc) in row.iter().enumerate() {
                if !loc || (x == centre_x && y == centre_y) {
                    continue;
                }
                let y_dist = y as f32 - centre_y as f32;
                let x_dist = x as f32 - centre_x as f32;
                coords.push((
                    {
                        let gcd = (centre_y as i32 - y as i32).gcd(&(x_dist as i32));
                        let num = -((((centre_y as i32 - y as i32)/gcd) as f32).atan2((x_dist as i32/gcd) as f32) - f32::consts::FRAC_PI_2);
                        if num < 0.0 {
                            num + 2.0 * f32::consts::PI
                        } else {
                            num
                        }
                    },
                    (y_dist.powf(2.0) + x_dist.powf(2.0)).sqrt(),
                    x,
                    y,
                    x_dist,
                    y_dist
                ));
            }
        }
        coords.sort_by(|a,b| {
            if a.0 != b.0 {
                return a.0.partial_cmp(&b.0).unwrap();
            }
            return a.1.partial_cmp(&b.1).unwrap();
        });

        let mut coord = coords[0];
        for c in &coords {
            if c.0 > prev_angle {
                coord = *c;
                break;
            }
        }
        map[coord.3][coord.2] = false;
        prev_angle = coord.0;

        println!("{}:{} = {}:{} @ {}", coord.2, coord.3, coord.4, coord.5, coord.0.to_degrees());
    }
}

/*
plotLine(int x0, int y0, int x1, int y1)
    dx =  abs(x1-x0);
    sx = x0<x1 ? 1 : -1;
    dy = -abs(y1-y0);
    sy = y0<y1 ? 1 : -1;
    err = dx+dy;  /* error value e_xy */
    while (true)   /* loop */
        if (x0==x1 && y0==y1) break;
        e2 = 2*err;
        if (e2 >= dy) 
            err += dy; /* e_xy+e_x > 0 */
            x0 += sx;
        end if
        if (e2 <= dx) /* e_xy+e_y < 0 */
            err += dx;
            y0 += sy;
        end if
    end while
    */

fn line(points: &mut HashSet<(usize, usize)>, map: &Vec<Vec<bool>>, mut x0: f32, mut y0: f32, mut x1: f32, mut y1: f32) {
    let dx = (x1 as i32- x0 as i32).abs();
    let sx = if x0 < x1 {1.0} else {-1.0};

    let dy = -(y1 as i32 - y0 as i32).abs();
    let sy = if y0 < y1 {1.0} else {-1.0};

    let mut err = dx + dy;

    if dx == 0 && dy == 0 {
        return;
    }

    if dy == 0 {
        loop {
            x0 += sx;
            if x0 as usize == map[0].len() || map[y0 as usize][x0 as usize] {
                break;
            } 
        }
        points.insert((x0 as usize, y0 as usize));
        return;
    }
    if dx == 0 {
        loop {
            y0 += sy;
            if y0 as usize == map.len() || map[y0 as usize][x0 as usize] {
                break;
            } 
        }
        points.insert((x0 as usize, y0 as usize));
        return;
    }

    x0 += 0.5;
    y0 += 0.5;
    x1 += 0.5;
    y1 += 0.5;

    let xinit = x0;
    let yinit = y0;
    let grad = (y1-y0)/(x1-x0);


    let shallow = (y1-y0).abs() < (x1-x0).abs();

    loop {
        if x0 == x1 && y0 == y1 {
            points.insert(((x0 - 0.5) as usize, (y0 - 0.5) as usize));
            break;
        }
        let e2 = 2*err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
        if map[y0 as usize][x0 as usize] && 
            x0%1.0 == 0.5 && (
                ((grad*(x0-xinit)+yinit)%1.0 == 0.5 && shallow)
                || (((y0-yinit)/grad+xinit)%1.0 == 0.5 && !shallow))
        {
            points.insert(((x0 - 0.5) as usize, (y0 - 0.5) as usize));
            break;
        }
    }
}