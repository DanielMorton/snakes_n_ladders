use crate::util::print_hms;
use clap::{value_parser, Args};
use rand::Rng;
use rayon::prelude::*;
use std::time::Instant;

#[derive(Args)]
pub struct OctoArgs {
    #[arg(short = 'n', required = true, value_parser = value_parser!(u64))]
    num_iterations: u64,

    #[arg(long = "end", required = true, value_parser = value_parser!(i64))]
    end: i64,
}

fn print(moves: &[u64]) {
    let min_moves = moves.iter().min().expect("Move array empty.");
    let min_moves_count = moves
        .iter()
        .filter(|&m| m == min_moves)
        .collect::<Vec<_>>()
        .len();
    let move_sum: f64 = moves.iter().sum::<u64>() as f64;
    let count = moves.len() as f64;
    let mean = move_sum / count;

    let sum_squares: f64 = moves.iter().map(|&x|  x.pow(2)).sum::<u64>() as f64;
    let variance = (sum_squares / count) - (mean * mean);
    let std = f64::sqrt(variance);
    let sum_cubes: f64 = moves.iter().map(|&x| x.pow(3)).sum::<u64>() as f64;
    let mean_cubes = sum_cubes/count;
    let m3 = mean_cubes - 3.0 * mean * variance - mean.powi(3);
    let skew = m3/std.powi(3);

    println!("Shortest Path Length: {}", min_moves);
    println!(
        "Shortest Path Fraction: {}",
        (min_moves_count as f64) / (moves.len() as f64)
    );

    println!("Mean moves: {}", mean);
    println!("Variance: {}", variance);
    println!("Standard deviation: {}", std);
    println!("Skew: {}", skew);
    println!("Mean Cubes: {}", mean_cubes)
}

pub fn octo_sim(args: OctoArgs) {
    let num_iterations = args.num_iterations;
    let end = args.end;

    let start_time = Instant::now();

    let moves: Vec<u64> = (0..num_iterations)
        .into_par_iter()
        .map(|_| simulate_single_path(end))
        .collect();

    print_hms(&start_time);

    print(&moves);
}

fn simulate_single_path(n: i64) -> u64 {
    let mut rng = rand::thread_rng();
    let mut x = -n;
    let mut count = 0;

    while x != n {
        // Create a vector of possible values excluding current absolute value
        let abs_x = x.abs();

        // Generate a random value between 1 and n, excluding abs(x)
        x = loop {
            let val = rng.gen_range(1..=n);
            if val != abs_x {
                break val;
            }
        };

        // Randomly choose sign
        x = if rng.gen_bool(0.5) { x } else { -x };
        count += 1;
    }
    count
}
