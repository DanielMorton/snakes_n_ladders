use crate::util::print_hms;
use clap::{value_parser, Arg, Command};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;
use std::sync::Arc;
use std::time::Instant;

pub fn cube_sim() {
    let matches = Command::new("cube")
        .arg(
            Arg::new("N")
                .short('N')
                .required(true)
                .value_parser(value_parser!(u64)),
        )
        .arg(
            Arg::new("dim")
                .long("dim")
                .required(true)
                .value_parser(value_parser!(u8)),
        )
        .arg(
            Arg::new("start")
                .long("start")
                .required(false)
                .value_parser(value_parser!(usize)),
        )
        .arg(
            Arg::new("end")
                .long("end")
                .required(false)
                .value_parser(value_parser!(usize)),
        )
        .get_matches();
    let N = match matches.get_one::<u64>("N") {
        Some(&n) => n,
        None => panic!("Number of Iterations not specified."),
    };
    let dim = match matches.get_one::<u8>("dim") {
        Some(&d) => d,
        None => panic!("No dim specified."),
    };
    let start = match matches.get_one::<usize>("start") {
        Some(&s) => s,
        None => 0,
    };
    let end = match matches.get_one::<usize>("end") {
        Some(&s) => s,
        None => (1 << dim) - 1,
    };
    let choice = Arc::new((0..dim).collect::<Vec<u8>>());
    let s = Instant::now();
    let moves = (0..N)
        .into_par_iter()
        .map(|_| {
            let (mut corner, mut move_count) = (start, 0);
            let mut rng = thread_rng();
            let mut c;
            while corner != end {
                move_count += 1;
                c = match choice.choose(&mut rng) {
                    Some(&n) => n,
                    None => panic!("No choice made."),
                };
                corner ^= 1 << c;
            }
            move_count
        })
        .collect::<Vec<_>>();
    print_hms(&s);
    let move_sum = moves.iter().sum::<u64>() as f64;
    let move_length_float = moves.len() as f64;
    let move_mean = move_sum / move_length_float;
    let e2 = moves.iter().map(|x| x * x).sum::<u64>() as f64;
    let move_var = e2 / move_length_float - move_mean * move_mean;
    println!("{}", move_mean);
    println!("{}", f64::sqrt(move_var))
}
