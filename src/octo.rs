use crate::stats::{SimulationError, SimulationStats};
use crate::util::print_hms;
use clap::{value_parser, Args};
use rand::Rng;
use rayon::prelude::*;
use std::time::Instant;

/// Command line arguments for the octopus simulation
#[derive(Args)]
pub struct OctoArgs {
    /// Number of simulation iterations to run
    #[arg(short = 'n', required = true, value_parser = value_parser!(u64))]
    num_iterations: u64,

    /// Target end value for the simulation
    #[arg(long = "end", required = true, value_parser = value_parser!(i64))]
    end: i64,
}

impl OctoArgs {
    /// Validate the command line arguments
    fn validate(&self) -> Result<(), SimulationError> {
        if self.num_iterations == 0 {
            return Err(SimulationError::InvalidIterationCount(self.num_iterations));
        }
        if self.end <= 0 {
            return Err(SimulationError::InvalidTarget(self.end));
        }
        Ok(())
    }
}

/// Run the octopus simulation with given arguments
pub fn octo_sim(args: OctoArgs) -> Result<(), SimulationError> {
    args.validate()?;

    let start_time = Instant::now();

    let moves: Vec<u64> = (0..args.num_iterations)
        .into_par_iter()
        .map(|_| simulate_single_path(args.end))
        .collect::<Result<Vec<_>, _>>()?;

    print_hms(&start_time);

    let stats = SimulationStats::from_moves(&moves)?;
    stats.print();

    Ok(())
}

/// Simulate a single path until reaching the target value
fn simulate_single_path(target: i64) -> Result<u64, SimulationError> {
    if target <= 0 {
        return Err(SimulationError::InvalidTarget(target));
    }

    let mut rng = rand::thread_rng();
    let mut current = -target;
    let mut moves = 0;

    while current != target {
        let abs_current = current.abs();

        // Generate a new value different from current absolute value
        current = loop {
            let new_val = rng.gen_range(1..=target);
            if new_val != abs_current {
                break new_val;
            }
        };

        // Randomly assign sign
        current = if rng.gen_bool(0.5) { current } else { -current };
        moves += 1;
    }

    Ok(moves)
}
