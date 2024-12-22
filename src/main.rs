use crate::cube::{cube_sim, CubeArgs};
use crate::shuffle::{shuffle_instance, shuffle_sim, ShuffleArgs};
use crate::snl::{snakes_n_ladders, snl_simulation, SnlArgs};
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
    #[command(name = "shuffle-sim")]
    ShuffleSim(ShuffleArgs),
    /// Run snakes and ladders simulation
    #[command(name = "snakes-ladders")]
    SnakesLadders(SnlArgs),
    #[command(name = "snl-simulation")]
    SnlSimulation(SnlArgs)
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Cube(args) => cube_sim(args),
        Commands::Shuffle(args) => shuffle_instance(args),
        Commands::ShuffleSim(args) => shuffle_sim(args),
        Commands::SnakesLadders(args) => snakes_n_ladders(args),
        Commands::SnlSimulation(args) => snl_simulation(args)
    }
}
