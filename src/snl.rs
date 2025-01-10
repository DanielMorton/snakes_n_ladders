use crate::stats::{SimulationError, SimulationStats};
use clap::{value_parser, Args};
use rand::prelude::*;
use rayon::prelude::*;
use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use std::time::Instant;

use crate::util::print_hms;

const BOARD_SIZE: usize = 100;
const DICE_SIDES: usize = 6;

#[derive(Args)]
pub struct SnlArgs {
    #[arg(short = 'n', required = true, value_parser = value_parser!(u64))]
    num_iterations: u64,

    #[arg(long = "start", required = false, value_parser = value_parser!(usize), default_value="0")]
    start: usize,
}

struct SnlStatistics {
    start: usize,
    statistics: SimulationStats,
}

impl SnlStatistics {
    fn new(start: usize, moves: &[u64]) -> Result<Self, SimulationError> {
        Ok(SnlStatistics {
            start,
            statistics: SimulationStats::from_values(moves)?,
        })
    }

    fn print(&self) {
        println!("Start: {}", self.start);
        self.statistics.print()
    }

    fn write_header(&self) -> String {
        self.statistics.write_header()
    }

    fn write_statistics(&self, file: &mut File) {
        writeln!(file, "{},{}", self.start, self.statistics.write()).expect("Failed to write data");
    }
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

fn create_transition_matrix(snl_map: &HashMap<usize, usize>) -> Result<Vec<Vec<f64>>, SimulationError> {
    let mut matrix = vec![vec![0.0; BOARD_SIZE + 1]; BOARD_SIZE + 1];

    for (i, row) in matrix.iter_mut().enumerate().take(BOARD_SIZE + 1) {
        if !snl_map.contains_key(&i) {
            for j in (i + 1)..=(i + DICE_SIDES) {
                let destination = *min(snl_map.get(&j).unwrap_or(&j), &BOARD_SIZE);
                row[destination] += 1.0 / DICE_SIDES as f64;
            }
        }
    }
    Ok(matrix)
}

pub fn snl_simulation(args: SnlArgs) -> Result<(), SimulationError> {
    let start_time = Instant::now();
    let filename = "snl_results.csv";
    let mut file = File::create(&filename).expect("Failed to create file");
    let snl_map = create_snakes_and_ladders_map();
    let transition_matrix = create_transition_matrix(&snl_map)?;
    let possible_positions = Arc::new((0..=BOARD_SIZE).collect::<Vec<usize>>());
    (0..=98).filter(|s| !snl_map.contains_key(s)).for_each(|s| {
        let moves = simulate_games(args.num_iterations, s, &transition_matrix, &possible_positions).expect("Simulation Failed");
        let stats = SnlStatistics::new(s, &moves).expect("Failed to Create Statistics");
        if s == 0 {
            writeln!(file, "{}", format!("start,{}", stats.write_header())).expect("Failed to write header");
        }
        stats.write_statistics(&mut file);
    });
    print_hms(&start_time);
    Ok(())
}

pub fn snakes_n_ladders(args: SnlArgs) -> Result<(), SimulationError> {
    let snl_map = create_snakes_and_ladders_map();
    let start_position = *snl_map.get(&args.start).unwrap_or(&args.start);

    let transition_matrix = create_transition_matrix(&snl_map)?;
    let possible_positions = Arc::new((0..=BOARD_SIZE).collect::<Vec<usize>>());

    let start_time = Instant::now();
    let moves = simulate_games(
        args.num_iterations,
        start_position,
        &transition_matrix,
        &possible_positions,
    )?;
    print_hms(&start_time);

    let stats = SnlStatistics::new(args.start, &moves)?;
    stats.print();
    Ok(())
}



fn simulate_games(
    num_iterations: u64,
    start_position: usize,
    transition_matrix: &[Vec<f64>],
    possible_positions: &Arc<Vec<usize>>,
) -> Result<Vec<u64>, SimulationError> {
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

            Ok(move_count)
        })
        .collect::<Result<Vec<_>, _>>()
}
