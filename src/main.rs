use std::fs;
use std::path::{Path, PathBuf};

use log::debug;
use log::error;
use log::info;

use clap::Parser;

use rayon::prelude::*;

mod json;
use json::{extract_json_from_json, output_json, remove_section_no};
use serde_json::Value;

use std::collections::BTreeMap;

use anyhow::Result;

/*
    RUST_LOG=debug cargo run
*/

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename of the JSON file to parse.
    #[arg(short, long)]
    filename: Option<String>,

    /// Directory name.
    #[arg(short, long)]
    dirname: Option<String>,

    /// Output JSON instead of plain text.
    #[arg(short, long, action)]
    json: bool,

    /// If specified, maximum number of files to process from directory.
    #[arg(short, long)]
    maxfiles: Option<usize>,

    /// Include the section names in the output.
    #[arg(short, long, action)]
    sectionnames: bool,

    /// Include the file names in the output.
    #[arg(long, action)]
    filenames: bool,

    /// Remove some stuff with hard-coded regular expressions.
    #[arg(short, long, action)]
    remove: bool,
}

// With trait bounds.
fn get_files_in_directory<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>> {
    let mut file_paths = Vec::new();

    let args = Args::parse();
    let mut counter: usize = args.maxfiles.unwrap_or(0);

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "xml" || ext == "json" {
                    debug!("Added {:?} to file list.", path);
                    file_paths.push(path);
                    
                    if counter > 0 {
                        counter -= 1;
                        if counter == 0 {
                            debug!("Reached file limit.");
                            break;
                        }
                    }

                }
            }
        }
    }

    Ok(file_paths)
}

// With and without par_iter()
// cargo run -- -d ~/Downloads/PMC010xxxxxx -m 10000 > /dev/null  129.77s user 3.32s system 836% cpu   15.91 total
// cargo run -- -d ~/Downloads/PMC010xxxxxx -m 10000 > /dev/null  109.70s user 2.96s system  97% cpu 1:55.92 total
fn main() -> Result<()> { //, Box<dyn std::error::Error>> {
    env_logger::init();

    let args = Args::parse();
    
    // Check if dirname is not none first. If it exists, we parse all the
    // files in the directory.
    if args.dirname.is_some() {
        let dirfiles = get_files_in_directory(args.dirname.unwrap());
        match dirfiles {
            Ok(files) => {
                // iter(), par_iter() {
                files.par_iter().for_each(|file| { // Note that the order is unknown.
                    debug!("Starting {}.", file.file_name().unwrap().to_str().unwrap());
                    match extract_json_from_json(file) {
                        Ok(texts) => {
                            let filename = file.file_name().unwrap().to_str().unwrap();
                            if args.json {
                                output_json(filename, texts);
                            } else {
                                output(filename, texts);
                            }
                            debug!("Output {} ok.", filename);
                        },
                        Err(e) => error!("Error reading or parsing {}: {}",
                            file.file_name().unwrap().to_str().unwrap(),
                            e),
                    }
                    
                });
            }
            Err(e) => error!("Failed to read directory: {}", e),
        }
    }

    // We supplied a single filename.
    if args.filename.is_some() {
        let path_name = args.filename.unwrap();

        match extract_json_from_json(path_name.clone()) {
            Ok(texts) => {
                if args.json {
                    output_json(&path_name, texts);
                } else {
                    output(&path_name, texts);
                }
            },
            Err(e) => error!("Error reading or parsing JSON: {}", e),
        }
        
    }
    
    Ok(())
}

// ================================================================
// Output
// ================================================================

fn output(filename: &str, texts: Value) {
    let args = Args::parse();

    println!("{}", texts);
    /*
    if texts.len() > 2 {
        for (section, text) in &texts {
            if args.filenames {
                print!("{:?}\t", filename);
            }
            if args.sectionnames {
                print!("{}\t", remove_section_no(section));
            }
            println!("{text}");
        }
    } else {
        info!("Only {} sections in {:?}, skipping.", texts.len(), filename);
    }
    */
}

