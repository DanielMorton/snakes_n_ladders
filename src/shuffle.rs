use clap::{value_parser, Arg, Command};
use itertools::equal;

fn shuffle(array: &[u32]) -> Vec<u32> {
    let m = array.len() / 2;
    let (start, end) = (&array[..m], &array[m..]);
    start
        .iter()
        .zip(end.iter())
        .map(|(&x, &y)| vec![x, y])
        .flatten()
        .collect::<Vec<_>>()
}

pub fn shuffle_count() {
    let matches = Command::new("cube")
        .arg(
            Arg::new("n")
                .short('n')
                .required(true)
                .value_parser(value_parser!(u64))
                .help("Number of iterations to run"),
        )
        .get_matches();
    let cards = 2 * *matches
        .get_one::<u32>("n")
        .expect("Number of cards divided by 2");
    let first_array = (1..=cards).collect::<Vec<_>>();
    let mut array = shuffle(&first_array);
    let mut count = 1;
    while !equal(&first_array, &array) {
        array = shuffle(&array);
        count += 1;
    }
    println!("Number of Shuffles: {}", count)
}
