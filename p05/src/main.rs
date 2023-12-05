use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

const STEPS: usize = 7;

fn main() -> Result<(), String> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        return Err(format!("Use {} input_file", args[0]));
    }
    let filename = &args[1];

    let re = Regex::new(r#"(\d+)"#).unwrap();

    let file = File::open(filename).unwrap();
    let mut lines_iter = io::BufReader::new(file).lines().map(Result::unwrap);
    let seeds_line = lines_iter.next().unwrap();
    let seeds = re.find_iter(&seeds_line)
        .map(|m| m.as_str())
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(lines_iter.next().unwrap(), "");
    assert_eq!(lines_iter.next().unwrap(), "seed-to-soil map:");

    let mut maps: [Vec<[i64; 3]>; STEPS] = Default::default();
    let mut step = 0;
    for line in lines_iter {
        if line.is_empty() {
            continue;
        }
        if line.contains("map:") {
            step += 1;
            continue;
        }
        let nums: [i64; 3] = re.find_iter(&line)
            .map(|m| m.as_str())
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
            .try_into().unwrap();
        maps[step].push(nums);
    }

    let mut res = None;
    for seed in &seeds {
        let mut val = *seed;
        for step in 0..STEPS {
            let mut found = false;
            for tri in &maps[step] {
                if let Some(s) = map(val, tri) {
                    //println!("step {step}: mapping {val} → {s}");
                    val = s;
                    found = true;
                    break;
                }
            }
            if !found {
                //println!("step {step}: leaving {val}");
            }
        }
        //println!("{val}");
        match res {
            None => res = Some((seed, val)),
            Some((_, v1)) =>
                if val < v1 {
                    res = Some((seed, val));
                }
        }
    }
    println!("{:?}", res.unwrap());

    // Part B

    let mut ranges = vec![];
    let mut it = seeds.into_iter();
    while let Some(start) = it.next() {
        let len = it.next().unwrap();
        ranges.push(start..(start+len));
    }
    //println!("Initial: {ranges:?}");
    for step in 0..STEPS {
        let mut new_ranges = vec![];
        for tri in &maps[step] {
            let mut rem_ranges = vec![];
            for range in ranges {
                let ovlap = overlap(&range, tri);
                if ovlap.is_empty() {
                    rem_ranges.push(range);
                } else {
                    if ovlap.start > range.start {
                        rem_ranges.push(range.start..ovlap.start);
                    }
                    if ovlap.end < range.end {
                        rem_ranges.push(ovlap.end..range.end);
                    }
                    new_ranges.push(shift_range(ovlap, tri[0] - tri[1]));
                }
            }
            ranges = rem_ranges;
        }
        new_ranges.append(&mut ranges);
        ranges = new_ranges;
        //println!("After step {step}: {ranges:?}");
    }

    let min = ranges.iter().min_by(|a, b| a.start.cmp(&b.start)).unwrap();
    println!("{}", min.start);

    Ok(())
}

fn map(val: i64, tri: &[i64; 3]) -> Option<i64> {
    if val >= tri[1] && val < tri[1] + tri[2] {
        Some(tri[0] + val - tri[1])
    } else {
        None
    }
}

use std::ops::Range;

fn overlap(r: &Range<i64>, tri: &[i64; 3]) -> Range<i64> {
    let start = std::cmp::max(r.start, tri[1]);
    let end = std::cmp::min(r.end, tri[1] + tri[2]);
    start..end
}

fn shift_range(r: Range<i64>, shift: i64) -> Range<i64> {
    (r.start + shift)..(r.end + shift)
}
