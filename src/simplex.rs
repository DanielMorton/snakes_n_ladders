use std::time::Instant;
use clap::{Args, value_parser};
use rand::Rng;
use rayon::prelude::*;
use crate::util::print_hms;

#[derive(Args)]
pub struct SimplexArgs {
    #[arg(short = 'n', required = true, value_parser = value_parser!(u64))]
    num_iterations: u64,

    #[arg(long = "dim", required = true, value_parser = value_parser!(u64))]
    dim: u64,
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
    let sum_cubes = moves.iter().map(|&x| x.pow(3)).sum::<u64>() as f64;
    let mean_cubes = sum_cubes/count;
    let skew = moves.iter().map(|&x| ((x as f64 - mean)/std).powi(3)).sum::<f64>()/count;

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

pub fn simplex_sim(args: SimplexArgs) {
    let num_iterations = args.num_iterations;
    let dim = args.dim;

    let start_time = Instant::now();

    let moves = (0..num_iterations)
        .into_par_iter()
        .map(|_| simulate_single_path(dim))
        .collect::<Vec<_>>();

    print_hms(&start_time);

    print(&moves);
}

fn simulate_single_path(n: u64) -> u64 {
    let mut rng = rand::thread_rng();
    let mut x = 0;
    let mut count = 0;

    while x != n - 1 {
        x = rng.gen_range(0..n);
        count += 1;
    }

    count
}