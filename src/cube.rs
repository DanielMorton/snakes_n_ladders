use clap::{value_parser, Args};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;
use std::{sync::Arc, time::Instant};
use crate::stats::{SimulationError, SimulationStats};

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

impl CubeArgs {
    /// Validate the command line arguments
    fn validate(&self) -> Result<(), SimulationError> {
        if self.num_iterations == 0 {
            return Err(SimulationError::InvalidIterationCount(self.num_iterations));
        }
        if self.dim <= 0 {
            return Err(SimulationError::InvalidTarget(self.dim as i64));
        }
        if self.start >= usize::from(2usize.pow(u32::from(self.dim))) {
            return Err(SimulationError::InvalidTarget(self.start as i64))
        }
        Ok(())
    }
}

pub fn cube_sim(args: CubeArgs) -> Result<(), SimulationError> {
    args.validate()?;

    let end = (1 << args.dim) - 1;
    let possible_moves = Arc::new((0..args.dim).collect::<Vec<u8>>());

    let start_time = Instant::now();

    let moves = (0..args.num_iterations)
        .into_par_iter()
        .map(|_| simulate_single_path(&possible_moves, args.start, end))
        .collect::<Result<Vec<_>, _>>()?;

    print_hms(&start_time);

    let stats = SimulationStats::from_moves(&moves)?;
    stats.print();

    Ok(())
}

fn simulate_single_path(possible_moves: &Arc<Vec<u8>>, start: usize, end: usize) -> Result<u64, SimulationError> {
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

    Ok(move_count)
}
