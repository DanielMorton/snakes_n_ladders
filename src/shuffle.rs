use clap::{value_parser, Args};
use itertools::equal;

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

pub fn shuffle_count(args: ShuffleArgs) {
    let cards = 2 * args.n;
    let first_array = (1..=cards).collect::<Vec<_>>();
    let mut array = shuffle(&first_array);
    let mut count = 1;
    while !equal(&first_array, &array) {
        array = shuffle(&array);
        count += 1;
    }
    println!("Number of Shuffles: {}", count)
}
