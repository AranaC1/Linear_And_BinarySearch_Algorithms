use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::time::Instant;
use std::env;


//  Data Generation
// ─────────────────────────────────────────────


fn generate_data(filename: &str, max: u64) -> io::Result<()> {
    let mut file = File::create(filename)?;
    for i in 1..=max {
        writeln!(file, "{}", i)?;
    }
    println!(" Generated {} numbers from 1 to {} -> \"{}\"", max, max, filename);
    Ok(())
}


//  File Loading
// ─────────────────────────────────────────────
fn load_data(filename: &str) -> io::Result<Vec<i64>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut data = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        match trimmed.parse::<i64>() {
            Ok(n) => data.push(n),
            Err(_) => eprintln!("Warning: skipping non-integer line: \"{}\"", trimmed),
        }
    }
    Ok(data)
}


//  Search Algorithms
// ─────────────────────────────────────────────

/// Linear Search
fn linear_search(data: &[i64], target: i64) -> (Option<usize>, usize) {
    let mut comparisons = 0;
    for (i, &val) in data.iter().enumerate() {
        comparisons += 1;
        if val == target {
            return (Some(i), comparisons);
        }
    }
    (None, comparisons)
}

/// Binary Search
fn binary_search(data: &[i64], target: i64) -> (Option<usize>, usize) {
    let mut low: usize = 0;
    let mut high: usize = data.len();
    let mut comparisons = 0;

    while low < high {
        let mid = low + (high - low) / 2;
        comparisons += 1;
        match data[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return (Some(mid), comparisons),
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid,
        }
    }
    (None, comparisons)
}


//  Timed Runner
// ─────────────────────────────────────────────
fn run_timed<F>(label: &str, data: &[i64], target: i64, search_fn: F) -> u128
where
    F: Fn(&[i64], i64) -> (Option<usize>, usize),
{
    let start = Instant::now();
    let (result, comparisons) = search_fn(data, target);
    let elapsed = start.elapsed();

    match result {
        Some(idx) => println!(
            "  {:<15} -> found at index {:>10}  |  comparisons: {:>10}  |  time: {:>10} µs",
            label, idx, comparisons, elapsed.as_micros()
        ),
        None => println!(
            "  {:<15} -> not found              |  comparisons: {:>10}  |  time: {:>10} µs",
            label, comparisons, elapsed.as_micros()
        ),
    // generate 
    // ──────────────────────────────────────────────────────

    if args.len() >= 2 && args[1] == "generate" {
        if args.len() != 4 {
            eprintln!("Usage: {} generate <filename> <max>", args[0]);
            eprintln!("Example: cargo run -- generate data.txt 1000000");
            std::process::exit(1);
        }
        let filename = &args[2];
        let max: u64 = args[3].parse().unwrap_or_else(|_| {
            eprintln!("Error: <max> must be a positive integer.");
            std::process::exit(1);
        });
        generate_data(filename, max)?;
        return Ok(());
    }

    // search 
    // ────────────────────────────────────────────────────────
    if args.len() >= 2 && args[1] == "search" {
        if args.len() != 4 {
            eprintln!("Usage: {} search <filename> <target>", args[0]);
            eprintln!("Example: cargo run -- search data.txt 999999");
            std::process::exit(1);
        }
        let filename = &args[2];
        let target: i64 = args[3].parse().unwrap_or_else(|_| {
            eprintln!("Error: <target> must be an integer.");
            std::process::exit(1);
        });

        println!("Loading data from \"{}\" …", filename);
        let data = load_data(filename)?;
        println!("Loaded {} integers.", data.len());
        println!("Searching for target = {}\n", target);

        println!("{:-<100}", "");
        let t_linear = run_timed("Linear Search", &data, target, linear_search);
        let t_binary = run_timed("Binary Search", &data, target, binary_search);
        println!("{:-<100}", "");

        // Simple comparison summary
        if t_linear > 0 && t_binary > 0 {
            let ratio = t_linear as f64 / t_binary as f64;
            println!("\nBinary search was {:.1}x faster than linear search.", ratio);
        }

        return Ok(());
    }

    Ok(())
}
