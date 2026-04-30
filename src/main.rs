pub mod algorithms;

use image::{DynamicImage};
use clap::{Parser, Subcommand};

// -- CLI Boilerplate --
// Define the structure of the CLI tool
#[derive(Parser)]
#[command(version, about)]
struct CLI {
    /// The path to the output file
    #[arg(short, long, global=true, default_value_t=String::from("moshd_output.png"))]
    output: String,

    #[command(subcommand)]
    command: Commands
}

// Define the subcommands of the CLI tool
#[derive(Subcommand)]
enum Commands {
    /// Vertically pixelated lines
    Relay {
        input: String,
        /// How many lines the the relay should last (default will estimate a number based on
        /// height)
        #[arg(short, long, default_value_t=0)]
        modifier: i32 
    },
    /// 3D mirror slice effect 
    Slice3d {
        input: String,
        /// How spaced out should the slices be
        #[arg(short, long, default_value_t=7)]
        modifier:i32
    }
}

// -- Execution --
fn main() {
    let args = CLI::parse();

    let oimg: DynamicImage;

    match args.command {
        Commands::Relay { input, modifier } => { oimg = algorithms::relay(input, modifier) },
        Commands::Slice3d { input, modifier } => { oimg = algorithms::slice_3d(input, modifier) }
    }

    oimg.save(&args.output);

    print!("File saved at: {}\n\n", &args.output);

}
