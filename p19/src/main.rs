use std::fs::File;
use std::io::{self, BufRead};
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
enum Rule {
    Gt(char, u32),
    Lt(char, u32),
    True
}

type Rates = [u32; 4];

#[derive(Clone, Debug)]
struct Range {
    min: Rates,
    max: Rates
}

impl Rule {
    fn part(var: &char) -> usize {
        "xmas".find(*var).unwrap()
    }

    fn holds_for(&self, r: &Rates) -> bool {
        match self {
            Rule::Lt(var, rhs) => r[Self::part(var)] < *rhs,
            Rule::Gt(var, rhs) => r[Self::part(var)] > *rhs,
            Rule::True => true
        }
    }

    fn split(&self, rng: Range) -> (Option<Range>, Option<Range>) {
        match self {
            Rule::True => (Some(rng), None),
            Rule::Lt(var, rhs) | Rule::Gt(var, rhs) => {
                let rhs = *rhs;
                let part = Self::part(var);
                let (lhs1, lhs2) = (rng.min[part], rng.max[part]);
                if let Rule::Lt(_, _) = self {
                    match (lhs1 < rhs, lhs2 < rhs) {
                        (true, true) => (Some(rng), None),
                        (false, false) => (None, Some(rng)),
                        (true, false) => {
                            let (mut r1, mut r2) = (rng.clone(), rng);
                            r1.max[part] = rhs - 1;
                            r2.min[part] = rhs;
                            (Some(r1), Some(r2))
                        },
                        _ => unreachable!()
                    }
                } else {
                    match (lhs1 > rhs, lhs2 > rhs) {
                        (true, true) => (Some(rng), None),
                        (false, false) => (None, Some(rng)),
                        (false, true) => {
                            let (mut r1, mut r2) = (rng.clone(), rng);
                            r1.min[part] = rhs + 1;
                            r2.max[part] = rhs;
                            (Some(r1), Some(r2))
                        },
                        _ => unreachable!()
                    }
                }
            }
        }
    }
}

impl Range {
    fn size(&self) -> u64 {
        let mut ret = 1;
        for i in 0..4 {
            ret *= (self.max[i] - self.min[i] + 1) as u64;
        }
        ret
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Use {} input_file", args[0]);
    }
    let filename = &args[1];
    let file = File::open(filename).unwrap();

    let mut it = io::BufReader::new(file).lines().map(Result::unwrap);

    let mut map = HashMap::new();
    while let Some(line) = it.next() {
        if line.is_empty() {
            break;
        }
        let open = line.find('{').unwrap();
        let close = line.find('}').unwrap();
        let name = line[0..open].to_owned();
        let instr_str = &line[(open + 1)..close];
        let instr = instr_str.split(',').map(|inst| {
            if let Some(colon) = inst.find(':') {
                let sign_pos = inst.find(&['<', '>']).unwrap();
                assert!(sign_pos == 1);
                let name = inst.as_bytes()[0] as char;
                let sign = inst.as_bytes()[sign_pos] as char;
                let val = inst[(sign_pos + 1)..colon].parse::<u32>().unwrap();
                let rule = match sign {
                    '<' => Rule::Lt(name, val),
                    '>' => Rule::Gt(name, val),
                    _ => panic!("sign {sign}")
                };
                let target = &inst[(colon + 1)..];
                (rule, target.to_owned())
            } else {
                (Rule::True, inst.to_owned())
            }
        }).collect::<Vec<_>>();
        map.insert(name, instr);
    }

    // Part A
    let mut total = 0;
    for line in it {
        let exprs = line[1..(line.len() - 1)].split(',').map(|part| {
            let (var, value) = part.split_once('=').unwrap();
            let value = value.parse::<u32>().unwrap();
            (var.as_bytes()[0] as char, value)
        }).collect::<Vec<_>>();
        assert!(exprs.iter().map(|x| x.0).collect::<Vec<_>>() == ['x', 'm', 'a', 's']);
        let rates: Rates = exprs.iter().map(|x| x.1).collect::<Vec<_>>().try_into().unwrap();

        let mut wf = "in";
        loop {
            for (rule, target) in map.get(wf).unwrap() {
                if rule.holds_for(&rates) {
                    wf = &target;
                    break;
                }
            }
            if wf == "A" {
                total += rates.into_iter().sum::<u32>();
                break;
            } else if wf == "R" {
                break;
            }
        }
    }
    println!("{total}");

    // Part B
    let mut queue = VecDeque::new();
    let mut total = 0;
    queue.push_back(("in", Range{min: [1, 1, 1, 1], max: [4000, 4000, 4000, 4000]}));
    while let Some((wf, mut rng)) = queue.pop_front() {
        for (rule, target) in map.get(wf).unwrap() {
            let (acc, rej) = rule.split(rng);
            if let Some(acc) = acc {
                //println!("{acc:?} -> {target}");
                if target == "A" {
                    total += acc.size();
                } else if target != "R" {
                    queue.push_back((target, acc));
                }
            }
            if let Some(rej) = rej {
                rng = rej;
            } else {
                break;
            }
        }
    }
    println!("{total}");
}
