use std::collections::HashSet;

pub fn day12b() {
    let mut position: [(i32, i32, i32); 4] = [(10, 15, 7), (15, 10, 0), (20, 12, 3), (0, -3, 13)];
    let mut velocity = [(0,0,0); 4];

    let mut states = HashSet::new();
    states.insert(position);

    let mut i = 0;
    loop {
        i += 1;
        for &idx_1 in &[0usize, 1, 2, 3] {
            for &idx_2 in &[0usize, 1, 2, 3][(idx_1+1)..] {
                if position[idx_1].0 > position[idx_2].0 {
                    velocity[idx_1].0 -= 1;
                    velocity[idx_2].0 += 1;
                } else if position[idx_1].0 < position[idx_2].0 {
                    velocity[idx_1].0 += 1;
                    velocity[idx_2].0 -= 1;
                }
            }
        }

        if velocity[0].0 == 0 &&
            velocity[1].0 == 0 &&
            velocity[2].0 == 0 &&
            velocity[3].0 == 0 {
            println!("{}", i);
            break;
        }

        for (idx_1, pos) in position.iter_mut().enumerate() {
            pos.0 += velocity[idx_1].0;
        }
    }

    let mut i = 0;
    loop {
        i += 1;
        for &idx_1 in &[0usize, 1, 2, 3] {
            for &idx_2 in &[0usize, 1, 2, 3][(idx_1+1)..] {
                if position[idx_1].1 > position[idx_2].1 {
                    velocity[idx_1].1 -= 1;
                    velocity[idx_2].1 += 1;
                } else if position[idx_1].1 < position[idx_2].1 {
                    velocity[idx_1].1 += 1;
                    velocity[idx_2].1 -= 1;
                }
            }
        }

        if velocity[0].1 == 0 &&
            velocity[1].1 == 0 &&
            velocity[2].1 == 0 &&
            velocity[3].1 == 0 {
            println!("{}", i);
            break;
        }

        for (idx_1, pos) in position.iter_mut().enumerate() {
            pos.1 += velocity[idx_1].1;
        }
    }

    let mut i = 0;
    loop {
        i += 1;
        for &idx_1 in &[0usize, 1, 2, 3] {
            for &idx_2 in &[0usize, 1, 2, 3][(idx_1+1)..] {
                if position[idx_1].2 > position[idx_2].2 {
                    velocity[idx_1].2 -= 1;
                    velocity[idx_2].2 += 1;
                } else if position[idx_1].2 < position[idx_2].2 {
                    velocity[idx_1].2 += 1;
                    velocity[idx_2].2 -= 1;
                }
            }
        }

        if velocity[0].2 == 0 &&
            velocity[1].2 == 0 &&
            velocity[2].2 == 0 &&
            velocity[3].2 == 0 {
            println!("{}", i);
            break;
        }

        for (idx_1, pos) in position.iter_mut().enumerate() {
            pos.2 += velocity[idx_1].2;
        }
    }

    // lcm of outputs * 2 = ans
}