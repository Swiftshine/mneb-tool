mod animate;
mod mneb;
use anyhow::Result;
use clap::{Parser, Subcommand};
use glob::glob;
use std::{fs, path::Path};

#[derive(Subcommand, Debug)]
enum Usage {
    Animate {
        filename: String,
        #[arg(short, long, default_value_t = 60.0f32)]
        framerate: f32,
    },
    Convert {
        filename: String,
        /// The name of the JSON value to output.
        #[arg(default_value_t = String::from("out.json"))]
        output_json: String,
        #[arg(short, long, default_value_t = String::from("out"))]
        /// If processing multiple files, the folder to output the files to.
        output_folder_name: String,
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
            if filename.contains('*') && filename.contains(".mneb")
            // to be entirely sure we're only rendering mneb files
            {
                // go through multiple files
                let mut mneb_files = Vec::new();
                for entry in glob(filename).expect("Failed to read glob pattern.") {
                    match entry {
                        Ok(path) => {
                            if let Ok(bytes) = fs::read(&path)
                                && let Ok(mneb_file) = mneb::MNEBFile::from_bytes(&bytes)
                                && mneb_file.has_curves()
                            {
                                let name = format!("{}", path.file_name().unwrap().display());

                                mneb_files.push((name, mneb_file));
                            }
                        }

                        Err(e) => {
                            println!("Error matching glob pattern: {:?}", e);
                        }
                    }
                }

                if !mneb_files.is_empty() {
                    // sort alphabetically before playing
                    mneb_files.sort_by(|a, b| a.0.cmp(&b.0));
                    animate::animate_files(mneb_files, *framerate);
                } else {
                    println!("No valid MNEB files found matching pattern: {}", filename);
                }
            } else {
                // play just the one
                let file = fs::read(filename)?;
                let mneb_file = mneb::MNEBFile::from_bytes(&file)?;
                if mneb_file.has_curves() {
                    let filename =
                        format!("{}", Path::new(filename).file_name().unwrap().display());
                    animate::animate_file(mneb_file, *framerate, filename);
                } else {
                    // nothing to do
                    println!("File does not have curves to render.");
                }
            }
        }

        Usage::Convert {
            output_json,
            filename,
            output_folder_name,
            pretty,
        } => {
            if filename.contains('*') && filename.contains(".mneb") {
                let mut mneb_files = Vec::new();
                for entry in glob(filename).expect("Failed to read glob pattern.") {
                    match entry {
                        Ok(path) => {
                            if let Ok(bytes) = fs::read(&path)
                                && let Ok(mneb_file) = mneb::MNEBFile::from_bytes(&bytes)
                            {
                                let name = format!("{}", path.file_name().unwrap().display());

                                mneb_files.push((name, mneb_file));
                            }
                        }

                        Err(e) => {
                            println!("Error matching glob pattern: {:?}", e);
                        }
                    }
                }

                for (name, mneb_file) in mneb_files {
                    let json = if *pretty {
                        serde_json::to_string_pretty(&mneb_file)?
                    } else {
                        serde_json::to_string(&mneb_file)?
                    };

                    if !fs::exists(output_folder_name)? {
                        fs::create_dir(output_folder_name)?;
                    }
                    fs::write(format!("{}/{}.json", output_folder_name, name), json)?;
                }
            } else {
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
    }

    Ok(())
}
