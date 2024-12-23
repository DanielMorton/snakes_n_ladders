use crate::util::print_hms;
use clap::{value_parser, Args};
use itertools::equal;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::fs::File;
use std::io::Write;
use std::time::Instant;

#[derive(Args)]
pub struct ShuffleArgs {
    #[arg(short = 'n', required = true, value_parser = value_parser!(u32))]
    n: u32,
}

fn shuffle(array: &[u32]) -> Vec<u32> {
    let m = array.len() / 2;
    let (start, end) = (&array[..m], &array[m..]);
    start
        .iter()
        .zip(end.iter())
        .flat_map(|(&x, &y)| vec![x, y])
        .collect::<Vec<_>>()
}

fn shuffle_count(cards: u32) -> u32 {
    let first_array = (1..=cards).collect::<Vec<_>>();
    let mut array = shuffle(&first_array);
    let mut count = 1u32;
    while !equal(&first_array, &array) {
        array = shuffle(&array);
        count += 1;
    }
    count
}

fn write_result(shuffles: &[u32]) {
    let filename = "shuffle_results.csv";

    // Create and write to file
    let mut file = File::create(&filename).expect("Failed to create file");

    // Write header
    writeln!(file, "cards, shuffles").expect("Failed to write header");

    // Write data
   shuffles.iter().enumerate().for_each(|(i, sc)| {
        writeln!(file, "{},{}", (i + 1) * 2, sc).expect("Failed to write data");
    });

    println!("Results written to {}", filename);
}

pub fn shuffle_instance(args: ShuffleArgs) {
    let cards = 2 * args.n;
    let count = shuffle_count(cards);
    println!("Number of Shuffles: {}", count)
}

pub fn shuffle_sim(args: ShuffleArgs) {
    let start_time = Instant::now();
    let max_cards = args.n;
    let shuffles = (1..=max_cards)
        .into_par_iter()
        .map(|c| shuffle_count(2 * c))
        .collect::<Vec<_>>();
    write_result(&shuffles);
    print_hms(&start_time)
}
