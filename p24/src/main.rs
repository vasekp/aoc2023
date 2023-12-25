use std::io::{self, BufRead};

#[derive(Debug, PartialEq, PartialOrd)]
struct Stone {
    pos: [f64; 3],
    vel: [f64; 3]
}

impl Stone {
    fn intersection_2d(&self, other: &Stone) -> Option<[f64; 2]> {
        // p1 + t*v1 = p2 + s*v2
        // → (v1 v2).(t -s) = (p2-p1)
        let diff = [other.pos[0] - self.pos[0], other.pos[1] - self.pos[1]];
        let v1 = self.vel;
        let v2 = other.vel;
        let det = v1[0] * v2[1] - v1[1] * v2[0];
        if det == 0.0 { return None; }
        let t = (v2[1] * diff[0] - v2[0] * diff[1]) / det;
        let s = -(v1[0] * diff[1] - v1[1] * diff[0]) / det;
        if t >= 0.0 && s >= 0.0 {
            Some([self.pos[0] + t * self.vel[0], self.pos[1] + t * self.vel[1]])
        } else { None }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 4 {
        panic!("Use {} input_file min max", args[0]);
    }
    let filename = &args[1];

    let min = args[2].parse::<f64>().unwrap();
    let max = args[3].parse::<f64>().unwrap();

    let file = std::fs::File::open(filename).unwrap();
    let stones = io::BufReader::new(file).lines().map(Result::unwrap)
        .map(|line| {
            let (pos, vel) = line.split_once(" @ ").unwrap();
            let pos: [f64; 3] = pos.split(", ").map(|num| num.trim().parse::<f64>().unwrap())
                .collect::<Vec<_>>().try_into().unwrap();
            let vel: [f64; 3] = vel.split(", ").map(|num| num.trim().parse::<f64>().unwrap())
                .collect::<Vec<_>>().try_into().unwrap();
            Stone{ pos, vel }
        }).collect::<Vec<_>>();

    let mut count = 0;
    for first in &stones {
        for second in &stones {
            if second <= first {
                continue;
            }
            let Some(is) = first.intersection_2d(second) else { continue };
            if is[0] >= min && is[0] <= max && is[1] >= min && is[1] <= max {
                //println!("{first:?}");
                //println!("{second:?}");
                //println!("{:?}", is);
                count += 1;
            }
        }
    }
    println!("{count}");

    // Part B

    // Observation: the system would be overdetermined for 4+ points, so if 3 points
    // line up, and the problem promises a solution exist, then all points will.

    // The three positions at times t1, t2, t3 need to come in an affine relation
    // α_1 (p1 + t*v1) + α_2 (p2 + t*v2) = α_3 (p3 + t3*v3)
    // α_1 + α_2 = α_3
    // the same relation, with the same α's, must be held for the times alone
    // α1 t_1 + α_2 t_2 = α_3 t_3
    // (α_1 + α_2 = α_3)
    // This t_i this is not linear but putting α_i t_i as a new unknown makes it so.

    // Thus we supplement the positions and velocities with two different weights,
    // p → (px, py, pz, 1, 0)
    // v → (vx, vy, vz, 0, 1)
    // and find the null space of
    // Σ α_i p_i + Σ β_i v_i = 0
    // Σ α_i = 0
    // Σ β_i = 0
    // to get t_i as β_i/α_i. (3rd index changes sign w.r.t. the above).
    // This is 5 equations (vector + 2 scalar) for 6 unknowns (α_1 through β_3) so
    // the solution space is 1-dimensional.

    // We will fork with transpose because it's easier to build. So this is indexed
    // as mx[x][y] instead of mx[y][x]!
    // Putting the 1's and 0's at the beginning gives better numerical stability.
    let mut mx = vec![];
    for s in &stones[0..3] {
        mx.push([1.0, 0.0, s.pos[0], s.pos[1], s.pos[2]]);
        mx.push([0.0, 1.0, s.vel[0], s.vel[1], s.vel[2]]);
    }

    // Just apply Gauss blindly. I could do this more smartly, using integer math,
    // but in honesty I simply used Mathematica to get the exact answer first so this
    // is just for completeness.
    for y1 in 0..5 {
        let pivot = mx[y1][y1];
        for x in 0..6 {
            mx[x][y1] /= pivot;
        }
        for y2 in 0..5 {
            if y2 == y1 { continue; }
            let m = mx[y1][y2];
            for x in 0..6 {
                mx[x][y2] -= m * mx[x][y1];
            }
        }
    }

    let t = [mx[5][1] / mx[5][0], mx[5][3] / mx[5][2], -1.0 / mx[5][4]];
    let x = [0, 1, 2].map(|j| [0, 1, 2].map(|i| stones[j].pos[i] + t[j] * stones[j].vel[i]));
    // We use the (0,1) pair to get velocity and (0) to get initial position, arbitrary choice.
    let v0 = [0, 1, 2].map(|i| (x[1][i] - x[0][i]) / (t[1] - t[0])).map(|f| f.round());
    let x0 = [0, 1, 2].map(|i| x[0][i] - t[0] * v0[i]).map(|f| f.round());

    // Check the solution due to use of rounding.
    for stone in &stones {
        let time = (stone.pos[0] - x0[0]) / (v0[0] - stone.vel[0]);
        assert_eq!([0, 1, 2].map(|i| stone.pos[i] + time * stone.vel[i]),
            [0, 1, 2].map(|i| x0[i] + time * v0[i]));
    }

    println!("{}", (x0[0] + x0[1] + x0[2]) as i64);
}
