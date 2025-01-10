use std::time::Instant;
use clap::{Args, value_parser};
use rand::Rng;
use rayon::prelude::*;
use crate::stats::{SimulationError, SimulationStats};
use crate::util::print_hms;

#[derive(Args)]
pub struct SimplexArgs {
    #[arg(short = 'n', required = true, value_parser = value_parser!(u64))]
    num_iterations: u64,

    #[arg(long = "dim", required = true, value_parser = value_parser!(u64))]
    dim: u64,
}

impl SimplexArgs {
    /// Validate the command line arguments
    fn validate(&self) -> Result<(), SimulationError> {
        if self.num_iterations == 0 {
            return Err(SimulationError::InvalidIterationCount(self.num_iterations));
        }
        if self.dim <= 0 {
            return Err(SimulationError::InvalidTarget(self.dim as i64));
        }
        Ok(())
    }
}

pub fn simplex_sim(args: SimplexArgs) -> Result<(), SimulationError> {
    args.validate()?;

    let start_time = Instant::now();

    let moves = (0..args.num_iterations)
        .into_par_iter()
        .map(|_| simulate_single_path(args.dim))
        .collect::<Result<Vec<_>, _>>()?;

    print_hms(&start_time);

    let stats = SimulationStats::from_moves(&moves)?;
    stats.print();

    Ok(())
}

fn simulate_single_path(n: u64) -> Result<u64, SimulationError> {
    let mut rng = rand::thread_rng();
    let mut x = 0;
    let mut count = 0;

    while x != n - 1 {
        x = rng.gen_range(0..n);
        count += 1;
    }

    Ok(count)
}