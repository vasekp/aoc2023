use std::fs::File;
use std::io::{self, BufRead};
use std::cell::Cell;

#[derive(Debug)]
struct Block {
    from: [u16; 3],
    to: [u16; 3],
    stable: Cell<bool>
}

impl Block {
    fn new_from(line: &str) -> Block {
        let (from_s, to_s) = line.split_once('~').unwrap();
        Block{
            from: from_s.split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>().try_into().unwrap(),
            to: to_s.split(',')
                .map(|s| s.parse::<u16>().unwrap() + 1)
                .collect::<Vec<_>>().try_into().unwrap(),
            stable: Default::default()
        }
    }

    fn supported_by(&self, other: &Block) -> bool {
        other.to[2] == self.from[2]
            && other.to[0] > self.from[0] && other.from[0] < self.to[0]
            && other.to[1] > self.from[1] && other.from[1] < self.to[1]
    }

    fn is_bottom(&self) -> bool {
        self.from[2] == 1
    }

    fn fall_once(&mut self) {
        self.from[2] -= 1;
        self.to[2] -= 1;
        if self.is_bottom() {
            self.stable.replace(true);
        }
    }
}


fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Use {} input_file", args[0]);
    }
    let filename = &args[1];
    let file = File::open(filename).unwrap();

    let mut blocks = io::BufReader::new(file).lines()
        .map(|l| Block::new_from(&l.unwrap()))
        .collect::<Vec<_>>();
    //println!("{:?}", blocks);

    // Fall down

    for block in &blocks {
        block.stable.replace(block.is_bottom());
    }

    let count = blocks.len();
    let mut count_stable = blocks.iter().filter(|b| b.stable.get()).count();
    while count_stable < count {
        loop {
            let added = blocks.iter().filter(|upper|
                !upper.stable.get() && blocks.iter().any(|lower|
                    lower.stable.get() && upper.supported_by(lower)))
                .collect::<Vec<_>>();
            if added.is_empty() {
                break;
            }
            for block in added {
                block.stable.replace(true);
            }
        }
        for block in &mut blocks {
            if !block.stable.get() {
                block.fall_once();
            }
        }
        count_stable = blocks.iter().filter(|b| b.stable.get()).count();
        //println!("{count_stable} {count}");
    }
    //println!("{:?}", blocks);

    // Part A

    for upper in &blocks {
        let support = blocks.iter().filter(|lower| upper.supported_by(lower)).collect::<Vec<_>>();
        if support.len() == 1 {
            support.first().unwrap().stable.replace(false);
        }
    }
    let free = blocks.iter().filter(|block| block.stable.get()).count();
    println!("{free}");

    // Part B

    let support = blocks.iter().enumerate().map(
        |(_, upper)| blocks.iter().enumerate().filter(
            |(_, lower)| upper.supported_by(lower)
        ).map(|(ix, _)| ix).collect::<Vec<_>>()
    ).collect::<Vec<_>>();
    //println!("{support:?}");

    let mut ans = 0;
    for ix in 0..count {
        for block in &blocks {
            block.stable.replace(true);
        }

        blocks[ix].stable.replace(false);
        loop {
            let added = (0..count).filter(|&ix|
                blocks[ix].stable.get()
                && !blocks[ix].is_bottom()
                && support[ix].iter().all(|jx| !blocks[*jx].stable.get()))
                .collect::<Vec<_>>();
            //println!("{added:?}");
            if added.is_empty() {
                break;
            }
            ans += added.len();
            added.into_iter().for_each(|ix| { blocks[ix].stable.replace(false); });
        }
    }
    println!("{ans}");
}
