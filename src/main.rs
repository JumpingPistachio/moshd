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
        /// How spaced out the slices should be (larger number, lesser effect)
        #[arg(short, long, default_value_t=7)]
        modifier:i32
    }
}

// -- Execution --
fn main() {
    let args = CLI::parse();
    let in_img: DynamicImage;
    let out_img: DynamicImage;

    // Get input value only
    let in_result = match &args.command {
        Commands::Relay { input, ..} => image::open(input),
        Commands::Slice3d { input, .. } => image::open(input)
    };

    // User-friendly error handling, ensures iimg is valid.
    match in_result {
        Ok(img) => {
            in_img = img;
        },
        Err(msg) => {
            println!("\nInput Error");
            println!("Error Message: {}\n", msg);
            return;
        }
    }
    
    match args.command {
        Commands::Relay { modifier, .. } => { out_img = algorithms::relay(in_img, modifier) },
        Commands::Slice3d { modifier, .. } => { out_img = algorithms::slice_3d(in_img, modifier) }
    }

    // User-friendly error handling, ensures oimg is actually saved
    match out_img.save(&args.output) {
        Ok(..) => {},
        Err(msg) => {
            println!("\nOutput Error");
            println!("Error Message: {}\n", msg);
            return;
        }
    }

    println!("File saved at: {}\n", &args.output);

}
