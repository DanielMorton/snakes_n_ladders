use clap::{value_parser, Args};
use rand::prelude::*;
use rayon::prelude::*;
use std::cmp::min;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use crate::util::print_hms;

const BOARD_SIZE: usize = 100;
const DICE_SIDES: usize = 6;

#[derive(Args)]
pub struct SnlArgs {
    #[arg(short = 'n', required = true, value_parser = value_parser!(u64))]
    num_iterations: u64,

    #[arg(long = "start", required = true, value_parser = value_parser!(usize), default_value="0")]
    start: usize,
}

pub fn snakes_n_ladders(args: SnlArgs) {
    let num_iterations = args.num_iterations;
    let snl_map = create_snakes_and_ladders_map();
    let start_position = *snl_map.get(&args.start).unwrap_or(&args.start);

    let transition_matrix = create_transition_matrix(&snl_map);
    let possible_positions = Arc::new((0..=BOARD_SIZE).collect::<Vec<usize>>());

    let start_time = Instant::now();
    let mut moves = simulate_games(
        num_iterations,
        start_position,
        &transition_matrix,
        &possible_positions,
    );
    moves.sort();
    print_hms(&start_time);

    print_statistics(&moves);
}

fn create_snakes_and_ladders_map() -> HashMap<usize, usize> {
    HashMap::from([
        (1, 38),
        (4, 14),
        (9, 31),
        (16, 6),
        (21, 42),
        (28, 84),
        (36, 44),
        (47, 26),
        (49, 11),
        (51, 67),
        (56, 53),
        (62, 19),
        (64, 60),
        (71, 91),
        (80, 100),
        (87, 24),
        (93, 73),
        (95, 75),
        (98, 78),
    ])
}

fn create_transition_matrix(snl_map: &HashMap<usize, usize>) -> Vec<Vec<f64>> {
    let mut matrix = vec![vec![0.0; BOARD_SIZE + 1]; BOARD_SIZE + 1];

    for (i, row) in matrix.iter_mut().enumerate().take(BOARD_SIZE + 1) {
        if !snl_map.contains_key(&i) {
            for j in (i + 1)..=(i + DICE_SIDES) {
                let destination = *min(snl_map.get(&j).unwrap_or(&j), &BOARD_SIZE);
                row[destination] += 1.0 / DICE_SIDES as f64;
            }
        }
    }
    matrix
}

fn simulate_games(
    num_iterations: u64,
    start_position: usize,
    transition_matrix: &[Vec<f64>],
    possible_positions: &Arc<Vec<usize>>,
) -> Vec<u64> {
    (0..num_iterations)
        .into_par_iter()
        .map(|_| {
            let mut rng = thread_rng();
            let mut move_count = 0u64;
            let mut current_position = start_position;

            while current_position != BOARD_SIZE {
                move_count += 1;
                current_position = *possible_positions
                    .choose_weighted(&mut rng, |&pos| transition_matrix[current_position][pos])
                    .expect("Error choosing next position");
            }

            move_count
        })
        .collect()
}

fn print_statistics(moves: &[u64]) {
    let max_moves = moves.iter().max().unwrap();
    let move_sum: f64 = moves.iter().sum::<u64>() as f64;
    let moves_length = moves.len() as f64;
    let squared_sum: f64 = moves.iter().map(|&x| x * x).sum::<u64>() as f64;

    let mean = move_sum / moves_length;
    let variance = squared_sum / moves_length - mean * mean;

    println!("Maximum moves: {}", max_moves);
    println!("Mean moves: {}", mean);
    println!("Standard deviation: {}", f64::sqrt(variance));

    println!("Median: {}", calculate_median(moves));
}

fn calculate_median(moves: &[u64]) -> f64 {
    let len = moves.len();
    if len % 2 == 0 {
        (moves[len / 2 - 1] + moves[len / 2]) as f64 / 2.0
    } else {
        moves[len / 2] as f64
    }
}
