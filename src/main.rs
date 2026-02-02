mod mneb;
use anyhow::Result;
// use clap::{Parser, ValueEnum};
use clap::Parser;
use std::fs;

// #[derive(Clone, Debug, ValueEnum)]
// enum Usage {
//     Graph,
//     Convert,
// }

#[derive(Parser, Debug)]
struct Args {
    // usage: Usage,
    mneb_file: String,
    output_json: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let file = fs::read(args.mneb_file)?;
    let mneb_file = mneb::MNEBFile::from_bytes(&file)?;
    println!("{:#?}", mneb_file);
    Ok(())
}
