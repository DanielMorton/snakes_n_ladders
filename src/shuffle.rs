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

/// Command line arguments for shuffle operations
#[derive(Args)]
pub struct ShuffleArgs {
    /// Number of card pairs (total cards will be 2 * n)
    #[arg(short = 'n', required = true, value_parser = value_parser!(u32))]
    n: u32,
}

/// Represents errors that can occur during shuffle operations
#[derive(Debug)]
pub enum ShuffleError {
    IoError(io::Error),
    InvalidInputError(String),
}

impl From<io::Error> for ShuffleError {
    fn from(error: io::Error) -> Self {
        ShuffleError::IoError(error)
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
/// * Number of shuffles needed to return to original order
fn shuffle_count(cards: u32) -> u32 {
    if cards % 2 != 0 {
        panic!("Number of cards must be even");
    }

    let original = (1..=cards).collect::<Vec<_>>();
    let mut current = shuffle(&original);
    let mut count = 1;

    while !equal(&original, &current) {
        current = shuffle(&current);
        count += 1;
    }
    count
}

/// Writes shuffle results to a CSV file
///
/// # Arguments
/// * `shuffles` - Vector of shuffle counts for different card numbers
/// * `filename` - Name of the output file
///
/// # Returns
/// * Result indicating success or failure
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
    let cards = 2 * args.n;
    if cards == 0 {
        return Err(ShuffleError::InvalidInputError("Number of cards must be greater than 0".to_string()));
    }

    let count = shuffle_count(cards);
    println!("Number of Shuffles: {}", count);
    Ok(())
}

/// Runs multiple shuffle simulations in parallel
pub fn shuffle_sim(args: ShuffleArgs) -> Result<(), ShuffleError> {
    let start_time = Instant::now();
    let max_cards = args.n;

    if max_cards == 0 {
        return Err(ShuffleError::InvalidInputError("Number of cards must be greater than 0".to_string()));
    }

    let shuffles: Vec<_> = (1..=max_cards)
        .into_par_iter()
        .map(|c| shuffle_count(2 * c))
        .collect();

    write_result(&shuffles, "shuffle_results.csv")?;
    print_hms(&start_time);
    Ok(())
}