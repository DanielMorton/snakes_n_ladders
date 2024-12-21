use clap::{value_parser, Args};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;
use std::{sync::Arc, time::Instant};

use crate::util::print_hms;

#[derive(Args)]
pub struct CubeArgs {
    #[arg(short = 'n', required = true, value_parser = value_parser!(u64))]
    num_iterations: u64,

    #[arg(long = "dim", required = true, value_parser = value_parser!(u8))]
    dim: u8,

    #[arg(long = "start", required = false, value_parser = value_parser!(usize), default_value="0")]
    start: usize,
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

    let sum_squares: f64 = moves.iter().map(|&x| x as f64 * x as f64).sum();
    let variance = (sum_squares / count) - (mean * mean);

    println!("Shortest Path Length: {}", min_moves);
    println!(
        "Shortest Path Fraction: {}",
        (min_moves_count as f64) / (moves.len() as f64)
    );

    println!("Mean moves: {}", mean);
    println!("Standard deviation: {}", f64::sqrt(variance));
}

pub fn cube_sim(args: CubeArgs) {
    let dim = args.dim;
    let num_iterations = args.num_iterations;
    let start = args.start;
    let end = (1 << dim) - 1;
    let possible_moves = Arc::new((0..dim).collect::<Vec<u8>>());

    let start_time = Instant::now();

    let moves: Vec<u64> = (0..num_iterations)
        .into_par_iter()
        .map(|_| simulate_single_path(&possible_moves, start, end))
        .collect();

    print_hms(&start_time);

    print(&moves);
}

fn simulate_single_path(possible_moves: &Arc<Vec<u8>>, start: usize, end: usize) -> u64 {
    let mut current_corner = start;
    let mut move_count = 0;
    let mut rng = thread_rng();

    while current_corner != end {
        let dimension = possible_moves
            .choose(&mut rng)
            .expect("Possible moves vector cannot be empty");
        current_corner ^= 1 << dimension;
        move_count += 1;
    }

    move_count
}
