use crate::cube::{cube_sim, CubeArgs};
use crate::octo::{octo_sim, OctoArgs};
use crate::shuffle::{shuffle_instance, shuffle_sim, ShuffleArgs};
use crate::simplex::{simplex_sim, SimplexArgs};
use crate::snl::{snakes_n_ladders, snl_simulation, SnlArgs};
use clap::{Parser, Subcommand}; // Added the necessary imports

mod cube;
mod octo;
mod shuffle;
mod simplex;
mod snl;
mod stats;
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
    Octo(OctoArgs),
    /// Run the shuffle counter
    Shuffle(ShuffleArgs),
    #[command(name = "shuffle-sim")]
    ShuffleSim(ShuffleArgs),
    Simplex(SimplexArgs),
    /// Run snakes and ladders simulation
    #[command(name = "snakes-ladders")]
    SnakesLadders(SnlArgs),
    #[command(name = "snl-simulation")]
    SnlSimulation(SnlArgs),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Cube(args) => cube_sim(args).expect("Hypercube Simulation."),
        Commands::Octo(args) => octo_sim(args).expect("Otoplex Simulation."),
        Commands::Shuffle(args) => shuffle_instance(args).expect("Card Shuffle"),
        Commands::ShuffleSim(args) => shuffle_sim(args).expect("Card Shuffle"),
        Commands::Simplex(args) => simplex_sim(args).expect("Simplex Simulation."),
        Commands::SnakesLadders(args) => {
            snakes_n_ladders(args).expect("Single Run Snakes and Ladders.")
        }
        Commands::SnlSimulation(args) => {
            snl_simulation(args).expect("Snakes and Ladders for All Starting Points")
        }
    }
}
