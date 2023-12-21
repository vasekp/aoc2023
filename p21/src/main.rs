use std::collections::{HashMap, VecDeque};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        panic!("Use {} input_file max_steps", args[0]);
    }
    let filename = &args[1];
    let max_steps = args[2].parse::<i32>().unwrap();
    let lines = std::fs::read_to_string(filename).unwrap()
        .lines().map(|line| line./*as_bytes().*/to_owned()).collect::<Vec<_>>();
    let h = lines.len();
    let w = lines[0].len();

    let mut map = HashMap::new();
    let mut queue = VecDeque::new();

    let start = lines.iter().enumerate()
        .fold(None, |opt, (row, line)|
            opt.or(line.find('S').map(|col| (row, col))))
        .unwrap();
    queue.push_back((start, 0));

    while let Some((pos, steps)) = queue.pop_front() {
        if map.contains_key(&pos) {
            continue;
        }
        if lines[pos.1].as_bytes()[pos.0] == b'#' {
            continue;
        }
        map.insert(pos, steps);
        let steps = steps + 1;
        if pos.0 > 0 { queue.push_back(((pos.0 - 1, pos.1), steps)); }
        if pos.0 < w - 1 { queue.push_back(((pos.0 + 1, pos.1), steps)); }
        if pos.1 > 0 { queue.push_back(((pos.0, pos.1 - 1), steps)); }
        if pos.1 < h - 1 { queue.push_back(((pos.0, pos.1 + 1), steps)); }
    }

    let count = map.into_iter().filter(|(_, steps)| *steps <= max_steps && *steps % 2 == max_steps % 2).count();
    println!("{count}");
}
