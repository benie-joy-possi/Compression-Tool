use std::fs;

use clap::Parser;
use cli::Cli;
use lz::{Token, compress_lz, decompress_lz};
use rle::{rle_compress, rle_decompress};

mod cli;
mod lz;
mod rle;
fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        cli::Commands::Compress {
            input_file,
            output_file,
            rle,
            lz,
        } => {
            let input_data = fs::read(&input_file)?;

            if rle {
                let compressed = rle_compress(&input_data);
                // println!("{:?}", compressed);

                let mut flat: Vec<u8> = Vec::new();

                for (count, value) in &compressed {
                    flat.push(*count);
                    flat.push((*value).into());
                }

                fs::write(output_file, flat)?;
            } else if lz {
                let compressed = compress_lz(&input_data);

                let mut flat: Vec<u8> = Vec::new();
                for token in &compressed {
                    match token {
                        Token::Literal(b) => {
                            flat.push(0);
                            flat.push(*b);
                        }
                        Token::Match { offset, length } => {
                            flat.push(1);
                            flat.push(*offset as u8);
                            flat.push(*length as u8);
                        }
                    }
                }
                fs::write(output_file, flat)?;
            }
        }
        cli::Commands::Decompress {
            input_file,
            output_file,
            rle,
            lz,
        } => {
            let input_data = fs::read(&input_file)?;

            if rle {
                let mut decoded = Vec::new();
                let mut i = 0;

                while i + 1 < input_data.len() {
                    let count = input_data[i];
                    let value = input_data[i + 1];
                    decoded.push((count, value));
                    i += 2;
                }

                let decompressed = rle_decompress(&decoded);
                fs::write(output_file, decompressed)?;
            } else if lz {
                let mut tokens = Vec::new();
                let mut i = 0;

                while i < input_data.len() {
                    match input_data[i] {
                        0 => {
                            tokens.push(Token::Literal(input_data[i + 1]));
                            i += 2;
                        }
                        1 => {
                            let offset = input_data[i + 1] as usize;
                            let length = input_data[i + 2] as usize;
                            tokens.push(Token::Match { offset, length });
                            i += 3;
                        }

                        _ => panic!("Invalid token marker in LZ stream"),
                    }
                }

                let decompressed = decompress_lz(&tokens);
                fs::write(output_file, decompressed)?;
            }
        }
    }
    Ok(())
}
