pub fn day12a() {
    let mut position: [(i32, i32, i32); 4] = [(10, 15, 7), (15, 10, 0), (20, 12, 3), (0, -3, 13)];
    let mut velocity = [(0,0,0); 4];

    for _ in 0..1000 {
        for (idx_1, _) in position.clone().iter().enumerate() {
            for (idx_2, _) in position.clone()[(idx_1+1)..].iter().enumerate() {
                if position[idx_1].0 > position[idx_2+idx_1+1].0 {
                    velocity[idx_1].0 -= 1;
                    velocity[idx_2+idx_1+1].0 += 1;
                } else if position[idx_1].0 < position[idx_2+idx_1+1].0 {
                    velocity[idx_1].0 += 1;
                    velocity[idx_2+idx_1+1].0 -= 1;
                }
                if position[idx_1].1 > position[idx_2+idx_1+1].1 {
                    velocity[idx_1].1 -= 1;
                    velocity[idx_2+idx_1+1].1 += 1;
                } else if position[idx_1].1 < position[idx_2+idx_1+1].1 {
                    velocity[idx_1].1 += 1;
                    velocity[idx_2+idx_1+1].1 -= 1;
                }
                if position[idx_1].2 > position[idx_2+idx_1+1].2 {
                    velocity[idx_1].2 -= 1;
                    velocity[idx_2+idx_1+1].2 += 1;
                } else if position[idx_1].2 < position[idx_2+idx_1+1].2 {
                    velocity[idx_1].2 += 1;
                    velocity[idx_2+idx_1+1].2 -= 1;
                }
            }
        }

        for (idx_1, pos) in position.iter_mut().enumerate() {
            pos.0 += velocity[idx_1].0;
            pos.1 += velocity[idx_1].1;
            pos.2 += velocity[idx_1].2;
        }
    }

    let mut energy = 0;
    for (i, pos) in position.iter().enumerate() {
        let mut pot = (pos.0).abs();
        pot += (pos.1).abs();
        pot += (pos.2).abs();

        let mut kin = (velocity[i].0).abs();
        kin += (velocity[i].1).abs();
        kin += (velocity[i].2).abs();

        energy += pot * kin;
    }

    println!("{}", energy);
}