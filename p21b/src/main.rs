use std::collections::{HashMap, VecDeque};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        panic!("Use {} input_file max_steps", args[0]);
    }
    let filename = &args[1];
    let max_steps = args[2].parse::<usize>().unwrap();
    let lines = std::fs::read_to_string(filename).unwrap()
        .lines().map(|line| line./*as_bytes().*/to_owned()).collect::<Vec<_>>();
    let sz = lines.len();
    assert!(lines[0].len() == sz);
    let sz32 = sz as i32;

    let mut map = HashMap::new();
    let mut queue = VecDeque::new();

    let start = lines.iter().enumerate()
        .fold(None, |opt, (row, line)|
            opt.or(line.find('S').map(|col| (row as i32, col as i32))))
        .unwrap();
    queue.push_back((start, 0));

    let mut last_steps = 0;
    let mut counts = [0, 0];
    let mut hist = [0; 3];
    let mut hist_estim = VecDeque::new();
    let hist_len = 3; // arbitrary!

    while let Some((pos, steps)) = queue.pop_front() {
        if map.contains_key(&pos) {
            continue;
        }
        if lines[pos.1.rem_euclid(sz32) as usize].as_bytes()[pos.0.rem_euclid(sz32) as usize] == b'#' {
            continue;
        }
        if steps > last_steps {
            // After a while, the expansion becomes perfectly quadratic every sz steps (though with
            // different constant terms depending on step % sz). This is because the same features repeat
            // on every diagonal front, one more per sz. The only irregularities are near the N, S, E, W
            // corners. So we study just those cases where last_steps === max_steps (mod sz).
            if last_steps % sz == max_steps % sz {
                //println!("{last_steps}: {}", counts[last_steps % 2]);
                hist = [hist[1], hist[2], counts[last_steps % 2]];
                // Sooner or later this prediction stabilizes at a constant; that's our answer.
                let estim = extrapol(&hist, last_steps, sz, max_steps);
                //println!("{estim}");
                hist_estim.push_back(estim);
                while hist_estim.len() > hist_len {
                    hist_estim.pop_front();
                }
                if hist_estim.len() == hist_len && hist_estim.iter().all(|x| x == hist_estim.front().unwrap()) {
                    break;
                }
            }
            last_steps = steps;
        }
        if steps > max_steps {
            break;
        }
        map.insert(pos, steps);
        counts[steps % 2] += 1;
        let steps = steps + 1;
        queue.push_back(((pos.0 - 1, pos.1), steps));
        queue.push_back(((pos.0 + 1, pos.1), steps));
        queue.push_back(((pos.0, pos.1 - 1), steps));
        queue.push_back(((pos.0, pos.1 + 1), steps));
    }

    // now hist_estim is hist_len equal elements
    println!("{}", hist_estim.front().unwrap());
}

fn extrapol(hist: &[usize; 3], cur: usize, delta: usize, target: usize) -> usize {
    // hist interpreted as f(-2), f(-1), f(0) with f assumed quadratic
    // need to remain within range (especially nonnegative) in every step!
    // f(-2) = 4α - 2β + γ
    // f(-1) =  α -  β + γ
    // f(0)  =           γ
    let &[v1, v2, v3] = hist;
    // f(-1) - f(-2) = β - 3α
    // f(0) - f(-1)  = β -  α
    let [d1, d2] = [v2 - v1, v3 - v2];
    // d2 - d1 = 2α
    let two_a = d2 - d1;
    // then we need f(x) where x = this:
    let eff_x = (target - cur) / delta;
    // f(x) = α x^2 + β x + γ
    //      = f(0) + x (α x + β)
    // Note that β is negative in our case!! We need to substitute:
    // 2d1 = 2β - 2α
    // f(x) = f(0) + x (2α x + 2β) / 2
    //      = f(0) + x (2α x + 2d1 + 2α) / 2
    eff_x * (two_a * (eff_x + 1) + 2 * d2) / 2 + hist[2]
}
