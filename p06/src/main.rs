fn main() -> Result<(), String> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        return Err(format!("Use {} input_file", args[0]));
    }
    let filename = &args[1];

    let content = std::fs::read_to_string(filename).unwrap();
    let (times, records) = content.split_once('\n').unwrap();
    let mut total = 1i64;
    for (time, record) in std::iter::zip(times.split_whitespace(), records.split_whitespace()).skip(1) {
        let time = time.parse::<i64>().unwrap();
        let record = record.parse::<i64>().unwrap();
        let t = time as f64;
        let r = record as f64;
        let cross = (t/2f64 - ((t/2f64 * t/2f64) - r).sqrt()).floor() as i64;
        let num_wins = (time - cross) - cross - 1;
        //println!("{num_wins}");
        total *= num_wins;
    }
    println!("{total}");

    let time = times.split_whitespace().skip(1)
        .collect::<Vec<_>>().join("")
        .parse::<i64>().unwrap();
    let record = records.split_whitespace().skip(1)
        .collect::<Vec<_>>().join("")
        .parse::<i64>().unwrap();
    //println!("{time} {record}");
    let t = time as f64;
    let r = record as f64;
    let cross = (t/2f64 - ((t/2f64 * t/2f64) - r).sqrt()).floor() as i64;
    let num_wins = (time - cross) - cross - 1;
    println!("{num_wins}");

    Ok(())
}
