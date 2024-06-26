use std::fs;
use std::path::{Path, PathBuf};

use env_logger::Builder;
use log::{debug, error, info, LevelFilter};

use clap::Parser;

use rayon::prelude::*;

mod json;
use json::{extract_json_from_json, output_json, OutputArticle, OutputChunk};
use std::collections::BTreeMap;
use std::collections::BTreeSet;

use anyhow::Result;
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

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
    #[arg(short = 'F', long, action)]
    filenames: bool,
    
    /// Sentence splitter
    #[arg(short = 'S', long, action)]
    sentences: bool,

    // Remove some stuff with hard-coded regular expressions.
    /*
    #[arg(short, long, action)]
    remove: bool,
     */
    
    /// Output only abbreviations
    #[arg(short, long, action)]
    abbreviations: bool,

    /// Allowed sections, separated by spaces
    #[clap(short = 'A', long, value_delimiter = ' ', num_args = 1..)]
    pub allowed: Vec<String>,
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
    //env_logger::init();
    Builder::new()
        .filter_level(LevelFilter::Info) //LevelFilter::max()) // Info
        .init();
    
    let args = Args::parse();
    info!("{:?}", args);

    let allowed: BTreeSet<String> = args.allowed.into_iter().collect();
    
    // Check if dirname is not none first. If it exists, we parse all the
    // files in the directory.

    // We should collect the abbreviations first, before printing to
    // prevent doubles.
    let abbreviations = Mutex::new(BTreeMap::new());

    if args.dirname.is_some() {
        let dirfiles = get_files_in_directory(args.dirname.unwrap());
        let file_counter = Arc::new(AtomicUsize::new(0));

        // Mutex needed.
        let oc = Mutex::new(OutputChunk { 
            articles: BTreeMap::new()
        });

        match dirfiles {
            Ok(files) => {
                // iter(), par_iter() {
                files.par_iter().for_each(|file| { // Note that the order is unknown.
                    let filename = file.file_name().unwrap().to_str().unwrap();
                    debug!("Starting {}.", filename);
                    match extract_json_from_json(file, filename, &allowed) {
                        Ok(texts) => {
                            if args.abbreviations == true {
                                let mut abbr = abbreviations.lock().unwrap();
                                add_abbreviations(&mut abbr, texts);
                            } else {
                                let pmid = texts.pmid.clone();
                                let mut oc1 = oc.lock().unwrap();
                                oc1.articles.insert(pmid, texts);
                            }
                            debug!("Output {} ok.", filename);
                            info!("Processed {} {}", filename, file_counter.load(Ordering::SeqCst));
                        },
                        Err(e) => error!("Error reading or parsing {}: {}",
                            file.file_name().unwrap().to_str().unwrap(),
                            e)
                    }
                    file_counter.fetch_add(1, Ordering::SeqCst);
                });
            }
            Err(e) => error!("Failed to read directory: {}", e)
        } // match dirfiles
        info!("Total files processed: {}", file_counter.load(Ordering::SeqCst));
        // output, and/or create chunks?
        let oc1: &OutputChunk = &*oc.lock().unwrap();
        if args.json {
            output_json(oc1);
        } else {
            for (pmid, article) in &oc1.articles {
                output(pmid, article);
            }
        }
    }

    // We supplied a single filename. Should output be OutputChunk or
    // OutputArticle?
    if args.filename.is_some() {
        let path_name = args.filename.unwrap();

        match extract_json_from_json(path_name.clone(), &path_name, &allowed) {
            Ok(texts) => {
                if args.abbreviations == true {
                    let mut abbr = abbreviations.lock().unwrap();
                    add_abbreviations(&mut abbr, texts);
                    //dbg!("Output abbreviations.");
                    //output_abbreviations(&path_name, texts);
                } else {
                    if args.json {
                        output_json(&texts);
                    } else {
                        output(&path_name, &texts);
                    }
                }
            },
            Err(e) => error!("Error reading or parsing JSON: {}", e),
        }
    }

    if args.abbreviations == true {
        let abbr = abbreviations.lock().unwrap();
        output_abbreviations(&abbr);
    }
    
    Ok(())
}

// ================================================================
// Output
// ================================================================

// Print section-type and text, with optinal filenames/section-types.
fn output(filename: &str, texts: &OutputArticle) {
    let args = Args::parse();
    
    let paragraphs = &texts.sentences;

    for par in paragraphs {
        if args.filenames == true {
            print!("{}\t", filename);
        }
        if args.sectionnames == true {
            let par_type = &par.r#type;
            print!("{}\t", par_type);
        }
        let par_text = &par.text;
        println!("{}", par_text); 
    }    
}

// Convert the Value to an OutputArticle, and add the abbreviations
// to the BTreeMap.
fn add_abbreviations(abbreviations: &mut BTreeMap<String, String>, article: OutputArticle) {
    let new_abbreviations = article.abbreviations;
    for (k, v) in new_abbreviations.into_iter() {
        abbreviations.entry(k.clone()).or_insert_with(String::new).push_str(&v);
    }
}

// Loop and print, they are sorted.
fn output_abbreviations(abbreviations: &BTreeMap<String, String>) {
    println!("output_abbreviations");
    for (key, value) in abbreviations.iter() {
        println!("{}\t{}", key, value);
    }
}

