mod animate;
mod mneb;
use anyhow::Result;
use clap::{Parser, ValueEnum};
use std::fs;

#[derive(Clone, Debug, ValueEnum)]
enum Usage {
    Animate,
    // Convert,
}

#[derive(Parser, Debug)]
struct Args {
    usage: Usage,
    mneb_file: String,
    output_json: Option<String>,
    framerate: Option<f32>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let file = fs::read(args.mneb_file)?;
    let mneb_file = mneb::MNEBFile::from_bytes(&file)?;

    match args.usage {
        Usage::Animate => {
            if mneb_file.has_curves() {
                let framerate = args.framerate.unwrap_or_else(|| 60.0f32);
                animate::play_animation(mneb_file, framerate);
            } else {
                // nothing to do
                println!("File does not have curves to render.");
            }
        }
    }

    Ok(())
}
