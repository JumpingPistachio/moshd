use image::{DynamicImage, ImageReader, ImageBuffer, Rgb};
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

// -- Algorithms / Functions --
fn relay(path: String, modifier: i32) -> DynamicImage {
    // Input image from path
    let iimg = ImageReader::open(path)
        .expect("Path missing or invalid").decode().expect("File decoding error");

    // Buffer (whole image) and Pixel (individual pixel) 
    let mut buf = iimg.into_rgb8();
    let mut pix_lead: Rgb<u8>;

    // Either use input or decide an "ideal" one based on image height + some math
    let n: u32;
    if modifier > 0 {
        n = modifier as u32; 
        } else { 
        let mut f = buf.height() as f32;
        f = f.sqrt()/4.0;
        n = f.ceil() as u32;
    }

    for x in 0..(buf.width()){
        pix_lead = *buf.get_pixel(x, 0);
        
        for y in 0..(buf.height()){
            if (y % n == 0){
                pix_lead = *buf.get_pixel(x, y);
            }
            buf.put_pixel(x, y, pix_lead); 
        }
    }

    print!("\nModifier: {}\n", n);
    return DynamicImage::ImageRgb8(buf);
}

fn slice_3d(path: String, modifier: i32) -> DynamicImage {
    // Input image from path
    let iimg = ImageReader::open(path)
        .expect("Path missing or invalid").decode().expect("File decoding error");

    // Buffer (whole image) and Pixel (individual pixel) 
    let mut buf = iimg.into_rgb8();
    let orig_buf = buf.clone();
    let mut pix_lead: Rgb<u8>;
    let width = buf.width()-1;
    let height = buf.height()-1;

    for x in 0..(width){
        if x % modifier as u32 == 0 {
            for y in 0..(height){
                pix_lead = *orig_buf.get_pixel(width-x, y);
                buf.put_pixel(x,y, pix_lead);
            } 
        }
        
    }

    return DynamicImage::ImageRgb8(buf);
}

// -- Execution --
fn main() {
    let args = CLI::parse();

    let oimg: DynamicImage;

    match args.command {
        Commands::Relay { input, modifier } => { oimg = relay(input, modifier) },
        Commands::Slice3d { input, modifier } => { oimg = slice_3d(input, modifier) }
    }

    oimg.save(&args.output);

    print!("File saved at: {}\n\n", &args.output);

}
