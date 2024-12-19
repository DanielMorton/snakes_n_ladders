use crate::util::print_hms;
use clap::{value_parser, Arg, Command};
use rand::prelude::SliceRandom;
use rand::prelude::*;
use rand::thread_rng;
use rayon::prelude::*;
use std::cmp::min;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

pub fn snakes_n_ladders() {
    let matches = Command::new("snl")
        .arg(
            Arg::new("N")
                .short('N')
                .required(true)
                .value_parser(value_parser!(u64)),
        )
        .arg(
            Arg::new("start")
                .long("start")
                .required(false)
                .value_parser(value_parser!(usize)),
        )
        .get_matches();
    let N = match matches.get_one::<u64>("N") {
        Some(&n) => n,
        None => panic!("Number of Iterations not specified."),
    };
    let snl: HashMap<usize, usize> = HashMap::from([
        (1, 38),
        (4, 14),
        (9, 31),
        (16, 6),
        (21, 42),
        (28, 84),
        (36, 44),
        (47, 26),
        (49, 11),
        (51, 67),
        (56, 53),
        (62, 19),
        (64, 60),
        (71, 91),
        (80, 100),
        (87, 24),
        (93, 73),
        (95, 75),
        (98, 78),
    ]);
    let start = *matches
        .get_one::<usize>("start")
        .map(|s| snl.get(s).unwrap_or(s))
        .unwrap_or(&0);
    let mut M = vec![vec![0.0; 101]; 101];
    let mut col;
    for i in 0..=100 {
        if !snl.contains_key(&i) {
            for j in (i + 1)..=(i + 6) {
                col = min(*snl.get(&j).unwrap_or(&j), 100);
                M[i][col] += 1.0 / 6.0;
            }
        }
    }
    let choice = Arc::new((0..=100).collect::<Vec<usize>>());
    let s = Instant::now();
    let mut moves = (0..N)
        .into_par_iter()
        .map(|_| {
            let mut rng = thread_rng();
            let mut move_count = 0u64;
            let mut v = start;
            while v != 100 {
                move_count += 1;
                v = match choice.choose_weighted(&mut rng, |&c| M[v][c]) {
                    Ok(&n) => n,
                    Err(e) => panic!("{:?}", e),
                };
            }
            move_count
        })
        .collect::<Vec<_>>();
    print_hms(&s);
    let move_length = moves.len();
    let move_length_float = move_length as f64;
    let moves_max = moves.iter().max().unwrap();
    let move_sum = moves.iter().sum::<u64>() as f64;
    let e2 = moves.iter().map(|x| x * x).sum::<u64>() as f64;
    let move_mean = move_sum / move_length_float;
    let move_var = e2 / move_length_float - move_mean * move_mean;
    println!("{}", moves_max);
    println!("{}", move_mean);
    println!("{}", f64::sqrt(move_var));

    let so = Instant::now();
    moves.sort();
    print_hms(&so);
    let median = if move_length % 2 == 0 {
        ((moves[move_length / 2] + moves[move_length / 2 + 1]) as f64) / 2.0
    } else {
        moves[move_length / 2] as f64
    };
    println!("{}", median)
}
