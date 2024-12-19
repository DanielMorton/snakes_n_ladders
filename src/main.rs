mod cube;
mod snl;
mod util;
mod shuffle;

use std::cmp::max;
use std::time::Instant;
use itertools::Itertools;
use crate::cube::cube_sim;
use crate::snl::snakes_n_ladders;

fn print_hms(start: &Instant) {
    let millis = start.elapsed().as_millis();
    let seconds = millis / 1000;
    let (hour, minute, second) = (seconds / 3600, (seconds % 3600) / 60, seconds % 60);
    println!("{:02}:{:02}:{:02}.{}", hour, minute, second, millis % 1000)
}

fn main() {
    snakes_n_ladders();
}
