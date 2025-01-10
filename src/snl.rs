use clap::{value_parser, Args};
use rand::prelude::*;
use rayon::prelude::*;
use std::{
    cmp::min,
    collections::HashMap,
    fs::File,
    io::{self, Write},
    sync::Arc,
    time::Instant,
};
use thiserror::Error;
use crate::stats::{SimulationError, SimulationStats};
use crate::util::print_hms;

/// Game constants
const BOARD_SIZE: usize = 100;
const DICE_SIDES: usize = 6;
const OUTPUT_FILENAME: &str = "snl_results.csv";

/// Custom error type for Snakes and Ladders specific errors
#[derive(Error, Debug)]
pub enum SnlError {
    #[error("Invalid board position: {0}")]
    InvalidPosition(usize),
    #[error("Invalid start position: {0}")]
    InvalidStart(usize),
    #[error("Failed to create transition matrix")]
    TransitionMatrixError,
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("Simulation error: {0}")]
    SimError(#[from] SimulationError),
}

/// Command line arguments for the Snakes and Ladders simulation
#[derive(Args)]
pub struct SnlArgs {
    /// Number of simulation iterations to run
    #[arg(short = 'n', required = true, value_parser = value_parser!(u64))]
    num_iterations: u64,

    /// Starting position on the board
    #[arg(long = "start", required = false, value_parser = value_parser!(usize), default_value = "0")]
    start: usize,
}

impl SnlArgs {
    /// Validate command line arguments
    fn validate(&self) -> Result<(), SnlError> {
        if self.start >= BOARD_SIZE {
            return Err(SnlError::InvalidStart(self.start));
        }
        Ok(())
    }
}

/// Statistics for a specific starting position
#[derive(Debug)]
struct SnlStatistics {
    start: usize,
    statistics: SimulationStats,
}

impl SnlStatistics {
    /// Create new statistics from simulation results
    fn new(start: usize, moves: &[u64]) -> Result<Self, SnlError> {
        Ok(Self {
            start,
            statistics: SimulationStats::from_values(moves)?,
        })
    }

    /// Print statistics to stdout
    fn print(&self) {
        println!("Start: {}", self.start);
        self.statistics.print();
    }

    /// Get CSV header string
    fn header(&self) -> String {
        format!("start,{}", self.statistics.header())
    }

    /// Write statistics to CSV file
    fn write_to_csv(&self, file: &mut File) -> io::Result<()> {
        writeln!(file, "{},{}", self.start, self.statistics.write())
    }
}

/// Represents the Snakes and Ladders game board
#[derive(Debug)]
struct GameBoard {
    transitions: HashMap<usize, usize>,
    transition_matrix: Vec<Vec<f64>>,
    possible_positions: Arc<Vec<usize>>,
}

impl GameBoard {
    /// Create a new game board with standard snakes and ladders
    fn new() -> Result<Self, SnlError> {
        let transitions = Self::create_snakes_and_ladders_map();
        let transition_matrix = Self::create_transition_matrix(&transitions)?;
        let possible_positions = Arc::new((0..=BOARD_SIZE).collect());

        Ok(Self {
            transitions,
            transition_matrix,
            possible_positions,
        })
    }

    /// Create the standard snakes and ladders map
    fn create_snakes_and_ladders_map() -> HashMap<usize, usize> {
        HashMap::from([
            (1, 38), (4, 14), (9, 31), (16, 6), (21, 42),
            (28, 84), (36, 44), (47, 26), (49, 11), (51, 67),
            (56, 53), (62, 19), (64, 60), (71, 91), (80, 100),
            (87, 24), (93, 73), (95, 75), (98, 78),
        ])
    }

    /// Create probability transition matrix for the game
    fn create_transition_matrix(
        transitions: &HashMap<usize, usize>,
    ) -> Result<Vec<Vec<f64>>, SnlError> {
        let mut matrix = vec![vec![0.0; BOARD_SIZE + 1]; BOARD_SIZE + 1];

        for (i, row) in matrix.iter_mut().enumerate().take(BOARD_SIZE + 1) {
            if !transitions.contains_key(&i) {
                for j in (i + 1)..=min(i + DICE_SIDES, BOARD_SIZE) {
                    let destination = *transitions.get(&j).unwrap_or(&j);
                    row[destination] += 1.0 / DICE_SIDES as f64;
                }
            }
        }

        Ok(matrix)
    }

    /// Simulate multiple games with given parameters
    fn simulate_games(
        &self,
        num_iterations: u64,
        start_position: usize,
    ) -> Result<Vec<u64>, SnlError> {
        if start_position >= BOARD_SIZE {
            return Err(SnlError::InvalidPosition(start_position));
        }

        let moves = (0..num_iterations)
            .into_par_iter()
            .map(|_| self.simulate_single_game(start_position))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(moves)
    }

    /// Simulate a single game from start to finish
    fn simulate_single_game(&self, start_position: usize) -> Result<u64, SnlError> {
        let mut rng = thread_rng();
        let mut moves = 0;
        let mut position = start_position;

        while position != BOARD_SIZE {
            moves += 1;
            position = *self.possible_positions
                .choose_weighted(&mut rng, |&pos| self.transition_matrix[position][pos])
                .map_err(|_| SnlError::TransitionMatrixError)?;
        }

        Ok(moves)
    }
}

/// Run full simulation and save results to CSV
pub fn snl_simulation(args: SnlArgs) -> Result<(), SnlError> {
    args.validate()?;
    let start_time = Instant::now();
    let mut file = File::create(OUTPUT_FILENAME)?;
    let board = GameBoard::new()?;

    // Run simulation for each valid starting position
    for start in 0..99 {
        if !board.transitions.contains_key(&start) {
            let moves = board.simulate_games(args.num_iterations, start)?;
            let stats = SnlStatistics::new(start, &moves)?;
            if start == 0 {
                writeln!(file, "{}", stats.header())?;
            }
            stats.write_to_csv(&mut file)?;
        }
    }

    print_hms(&start_time);
    Ok(())
}

/// Run simulation for a single starting position
pub fn snakes_n_ladders(args: SnlArgs) -> Result<(), SnlError> {
    args.validate()?;
    let board = GameBoard::new()?;
    let start_position = *board.transitions.get(&args.start).unwrap_or(&args.start);

    let start_time = Instant::now();
    let moves = board.simulate_games(args.num_iterations, start_position)?;
    print_hms(&start_time);

    let stats = SnlStatistics::new(args.start, &moves)?;
    stats.print();

    Ok(())
}