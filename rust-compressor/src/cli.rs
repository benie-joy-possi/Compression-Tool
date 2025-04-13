use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Compressor")]
#[command(about= "compress or doecompress files using RLE or LZ77 ", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
pub enum Commands {
    // compress a file
    Compress {
        input_file: String,
        output_file: String,

        #[arg(long)]
        rle: bool,

        #[arg(long)]
        lz: bool,
    },

    Decompress {
        input_file: String,
        output_file: String,

        #[arg(long)]
        rle: bool,

        #[arg(long)]
        lz: bool,
    },
}
