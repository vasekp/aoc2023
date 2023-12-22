use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Use {} input_file", args[0]);
    }
    let filename = &args[1];

    let file = File::open(filename).unwrap();
    let mut total = 0;
    for line in io::BufReader::new(file).lines().map(Result::unwrap) {
        let (row, nums) = line.split_once(' ').unwrap();
        let nums = nums
            .split(',').map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>();
        let ways = num_ways_old(row[..].as_bytes(), &nums[..]);
        // Conveniently check the validity of the "new" scheme on all the input we have, not just the total
        let ways2 = num_ways_new(row[..].as_bytes(), &nums[..]);
        assert_eq!(ways, ways2);
        total += ways;
    }
    println!("{total}");

    // Part B

    let file = File::open(filename).unwrap();
    let mut total = 0;
    for line in io::BufReader::new(file).lines().map(Result::unwrap) {
        let (row, nums) = line.split_once(' ').unwrap();
        let mut row = row.to_owned();
        row.push('?');
        let mut row = row.repeat(5);
        row.pop();
        let nums = nums
            .split(',').map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>()
            .repeat(5);
        let ways = num_ways_new(row[..].as_bytes(), &nums[..]);
        total += ways;
    }
    println!("{total}");
}

/** Brute-force, recursive approach. Appropriate for part A but too slow for part B.
  Complexity: O(r^n) */
fn num_ways_old(row: &[u8], nums: &[usize]) -> usize {
    if nums.is_empty() {
        if row.iter().any(|c| *c == b'#') {
            return 0usize;
        } else {
            return 1usize;
        }
    }
    let num = nums[0];
    if row.len() < num {
        return 0usize;
    }
    let mut ways = 0;
    for start in 0..=(row.len() - num) {
        let end = start + num;
        if row[start..end].iter().all(|c| *c == b'#' || *c == b'?') {
            if end == row.len() {
                ways += num_ways_old(&row[end..], &nums[1..]);
            } else if row[end] == b'.' || row[end] == b'?' {
                ways += num_ways_old(&row[(end + 1)..], &nums[1..]);
            } else {
                // zero
            }
        }
        if row[start] == b'#' {
            // the first num can't start *after* this point
            break;
        }
    }
    ways
}

/** An O(r*n) approach. Starting from the last number, we build a table of # ways that x'th group 
  can start on y'th position. */
fn num_ways_new(row: &[u8], nums: &[usize]) -> usize {
    // We always only need one previous row so no need to keep the while table.
    let mut cur = vec![];
    let mut it = nums.iter().rev();
    // Last group - perhaps this could have been merged with the main cycle.
    let num = it.next().unwrap();
    for start in 0..row.len() {
        // Here we build the first row (cur)
        let end = start + num;
        if end > row.len() {
            cur.push(0usize);
            continue;
        }
        // need to be allowed here and followed by empty spaces only
        if row[start..end].iter().all(|c| *c == b'#' || *c == b'?') &&
            row[end..row.len()].iter().all(|c| *c == b'.' || *c == b'?') {
                cur.push(1usize);
            } else {
                cur.push(0usize);
            }
    }
    for num in it {
        // Here we're building the next row, to replace cur at the end
        let mut next = vec![];
        for start in 0..row.len() {
            let end = start + num;
            if end > row.len() - 1 {
                next.push(0usize);
                continue;
            }
            // need to be allowed here
            if row[start..end].iter().all(|c| *c == b'#' || *c == b'?') {
                // If there's another #, the next group can't start LATER than that
                let next_req = row[end..row.len()].iter().position(|x| x == &b'#')
                    .map(|x| x + end + 1).unwrap_or(row.len());
                if next_req < end + 1 {
                    // Here the # follows the current group directly which is not allowed
                    next.push(0usize);
                } else {
                    // otherwise we sum over the possible starting points of the next group
                    next.push(cur[(end + 1)..next_req].iter().sum());
                }
            } else {
                next.push(0usize);
            }
        }
        cur = next;
    }
    let mut total = 0usize;
    for (ix, ch) in row.iter().enumerate() {
        total += cur[ix];
        // If there's a #, the FIRST group can't start LATER than that, so we stop.
        if *ch == b'#' {
            break;
        }
    }
    total
}
