use crate::cube::{cube_sim, CubeArgs};
use crate::octo::{octo_sim, OctoArgs};
use crate::shuffle::{shuffle_instance, shuffle_sim, ShuffleArgs};
use crate::snl::{snakes_n_ladders, snl_simulation, SnlArgs};
use clap::{Parser, Subcommand};
use crate::simplex::{simplex_sim, SimplexArgs}; // Added the necessary imports

mod cube;
mod octo;
mod shuffle;
mod snl;
mod util;
mod simplex;
mod stats;

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
        Commands::Shuffle(args) => shuffle_instance(args),
        Commands::ShuffleSim(args) => shuffle_sim(args),
        Commands::Simplex(args) => simplex_sim(args).expect("Simplex Simulation."),
        Commands::SnakesLadders(args) => snakes_n_ladders(args),
        Commands::SnlSimulation(args) => snl_simulation(args),
    }
}
