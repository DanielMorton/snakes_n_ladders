use crate::util::print_hms;
use clap::{Args, value_parser};
use itertools::equal;
use rayon::prelude::*;
use std::{
    fs::File,
    io::{self, Write},
    path::Path,
    time::Instant,
};
use thiserror::Error;

/// Represents errors that can occur during shuffle operations
#[derive(Error, Debug)]
pub enum ShuffleError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("Invalid Input: {0} must be greater than zero")]
    InvalidInputError(u32),
}

/// Command line arguments for shuffle operations
#[derive(Args)]
pub struct ShuffleArgs {
    /// Number of card pairs (total cards will be 2 * n)
    #[arg(short = 'n', required = true, value_parser = value_parser!(u32))]
    n: u32,
}

impl ShuffleArgs {
    /// Validate command line arguments
    fn validate(&self) -> Result<(), ShuffleError> {
        if self.n <= 0 {
            return Err(ShuffleError::InvalidInputError(self.n));
        }
        Ok(())
    }
}

/// Performs a perfect shuffle on an array of numbers
///
/// A perfect shuffle splits the array in half and interleaves the elements
fn shuffle(array: &[u32]) -> Vec<u32> {
    let mid = array.len() / 2;
    let (left, right) = array.split_at(mid);

    left.iter()
        .zip(right.iter())
        .flat_map(|(&x, &y)| [x, y])
        .collect()
}

/// Counts how many shuffles are needed to return to the original order
///
/// # Arguments
/// * `cards` - Total number of cards (must be even)
///
/// # Returns
/// * Result with number of shuffles needed or error if input is invalid
fn shuffle_count(cards: u32) -> Result<u32, ShuffleError> {
    let original = (1..=cards).collect::<Vec<_>>();
    let mut current = shuffle(&original);
    let mut count = 1;

    while !equal(&original, &current) {
        current = shuffle(&current);
        count += 1;
    }
    Ok(count)
}

/// Writes shuffle results to a CSV file
///
/// # Arguments
/// * `shuffles` - Vector of shuffle counts for different card numbers
/// * `filename` - Name of the output file
///
/// # Returns
/// * Result indicating success or failure with error message
fn write_result(shuffles: &[u32], filename: &str) -> Result<(), ShuffleError> {
    let path = Path::new(filename);
    let mut file = File::create(path)?;

    writeln!(file, "cards,shuffles")?;

    for (i, &count) in shuffles.iter().enumerate() {
        writeln!(file, "{},{}", (i + 1) * 2, count)?;
    }

    println!("Results written to {}", filename);
    Ok(())
}

/// Runs a single shuffle simulation
pub fn shuffle_instance(args: ShuffleArgs) -> Result<(), ShuffleError> {
    args.validate()?;
    let cards = 2 * args.n;

    let count = shuffle_count(cards)?;
    println!("Number of Shuffles: {}", count);
    Ok(())
}

/// Runs multiple shuffle simulations in parallel
pub fn shuffle_sim(args: ShuffleArgs) -> Result<(), ShuffleError> {
    args.validate()?;
    let start_time = Instant::now();
    let max_cards = args.n;

    let shuffles = (1..=max_cards)
        .into_par_iter()
        .map(|c| shuffle_count(2 * c))
        .collect::<Result<Vec<_>, _>>()?;

    write_result(&shuffles, "shuffle_results.csv")?;
    print_hms(&start_time);
    Ok(())
}