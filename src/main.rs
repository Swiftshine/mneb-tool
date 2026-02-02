mod animate;
mod mneb;
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::fs;

#[derive(Subcommand, Debug)]
enum Usage {
    Animate {
        filename: String,
        #[arg(short, long, default_value_t = 60.0f32)]
        framerate: f32,
    },
    Convert {
        filename: String,
        #[arg(default_value_t = String::from("out.json"))]
        output_json: String,
        /// Make the JSON output pretty.
        #[arg(short, long)]
        pretty: bool,
    },
}

#[derive(Parser, Debug)]
#[command(subcommand_precedence_over_arg = true)]
struct Args {
    /// The action to perform.
    #[command(subcommand)]
    usage: Usage,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match &args.usage {
        Usage::Animate {
            framerate,
            filename,
        } => {
            let file = fs::read(filename)?;
            let mneb_file = mneb::MNEBFile::from_bytes(&file)?;
            if mneb_file.has_curves() {
                animate::play_animation(mneb_file, *framerate);
            } else {
                // nothing to do
                println!("File does not have curves to render.");
            }
        }

        Usage::Convert {
            output_json,
            filename,
            pretty,
        } => {
            let file = fs::read(filename)?;
            let mneb_file = mneb::MNEBFile::from_bytes(&file)?;
            let json = if *pretty {
                serde_json::to_string_pretty(&mneb_file)?
            } else {
                serde_json::to_string(&mneb_file)?
            };
            fs::write(output_json, json)?;
        }
    }

    Ok(())
}
