use crate::cube::{cube_sim, CubeArgs};
use crate::shuffle::{shuffle_count, ShuffleArgs};
use crate::snl::{snakes_n_ladders, SnlArgs};
use clap::{Parser, Subcommand}; // Added the necessary imports

mod cube;
mod shuffle;
mod snl;
mod util;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the cube simulation
    Cube(CubeArgs),
    /// Run the shuffle counter
    Shuffle(ShuffleArgs),
    /// Run snakes and ladders simulation
    #[command(name = "snakes-ladders")]
    SnakesLadders(SnlArgs),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Cube(args) => cube_sim(args),
        Commands::Shuffle(args)=> shuffle_count(args),
        Commands::SnakesLadders(args) => snakes_n_ladders(args),
    }
}
