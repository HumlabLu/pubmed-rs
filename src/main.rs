use std::fs;
use std::path::{Path, PathBuf};

use log::debug;
use log::error;
use log::info;
use log::warn;

use clap::Parser;

use roxmltree::Document;
use rayon::prelude::*;

mod json;
use json::extract_text_from_json;

use std::collections::BTreeMap;

use anyhow::Result;

/*
    RUST_LOG=debug cargo run
*/

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename of the XML file to parse.
    #[arg(short, long)]
    filename: Option<String>,

    /// Directory name.
    #[arg(short, long)]
    dirname: Option<String>,

    /// Maximum number of files to process.
    #[arg(short, long)]
    maxfiles: Option<usize>,

    /// Include the section names in the output.
    #[arg(short, long, action)]
    sectionnames: bool,

    /// Include the file names in the output.
    #[arg(long, action)]
    filenames: bool,

    /// Remove some stuff with regular expressions.
    #[arg(short, long, action)]
    remove: bool,
}

// With trait bounds.
fn get_files_in_directory<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>> {
    let mut file_paths = Vec::new();

    let args = Args::parse();
    let mut counter:usize = 0;
    if let Some(count) = args.maxfiles {
        counter = count;
    }
    
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "xml" || ext == "json" {
                    file_paths.push(path);
                    
                    if counter > 0 {
                        counter -= 1;
                        if counter == 0 {
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
    debug!("This is test output from debug!");
    //error!("{}", "This is test output from error!");
    info!("{:?}", "This is test output from info!");
    warn!("{:#?}", "This is test output from warn!");
    
    let args = Args::parse();
    
    // Check if dirname is not none first.
    if args.dirname.is_some() {
        let dirfiles = get_files_in_directory(args.dirname.unwrap());
        match dirfiles {
            Ok(files) => {
                // iter(), par_iter() {
                files.par_iter().for_each(|file| {
                    match extract_text_from_json(file) {
                        Ok(texts) => {
                            output(file.file_name().unwrap().to_str().unwrap(), texts);
                        },
                        Err(e) => error!("Error reading or parsing {}: {}",
                            file.file_name().unwrap().to_str().unwrap(),
                            e),
                    }

                    if false {
                        match extract_text_from_sec(file) {
                            Ok(sections) => {
                                for (title, texts) in sections {
                                    println!("{:?} Title: {}", file, title);
                                    for text in texts {
                                        println!("{:?} Text: {}", file, text);
                                    }
                                    println!("---"); // Separator for different sections
                                }
                            }
                            Err(e) => error!("Error: {}", e),
                        }
                    }
                    
                });
            }
            Err(e) => error!("Failed to read directory: {}", e),
        }
    }

    if args.filename.is_some() {
        let path_name = args.filename.unwrap();

        match extract_text_from_json(path_name.clone()) {
            Ok(texts) => {
                output(&path_name, texts);
            },
            Err(e) => error!("Error reading or parsing JSON: {}", e),
        }

        if false {
            match extract_text_from_sec(path_name) {
                Ok(sections) => {
                    for (title, texts) in sections {
                        println!("Title: {}", title);
                        for text in texts {
                            println!("Text: {}", text);
                        }
                        println!("---"); // Separator for different sections.
                    }
                }
                Err(e) => error!("Error: {}", e),
            }
        }
        
    }
    
    Ok(())
}

// ================================================================
// Output
// ================================================================

fn output(filename: &str, texts: BTreeMap<String, String>) {
    let args = Args::parse();

    if texts.len() > 2 {
        for (section, text) in &texts {
            if args.filenames {
                print!("{:?}\t", filename);
            }
            if args.sectionnames {
                print!("{section}\t");
            }
            println!("{text}");
        }
    } else {
        info!("Only {} sections.", texts.len());
    }
}

// ================================================================
// roxmltree
// Unused after switch to JSON.
// ================================================================

fn _extract_text_from_sec_tags(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let xml_content = fs::read_to_string(file_path)?;
    let doc = Document::parse(&xml_content)?;

    let mut texts = Vec::new();
    for node in doc.descendants().filter(|n| n.has_tag_name("sec")) {
        if let Some(text) = node.text() {
            texts.push(text.to_string());
        }
    }

    Ok(texts)
}

fn _extract_text_from_p_tags_in_sec(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let xml_content = fs::read_to_string(file_path)?;
    let doc = Document::parse(&xml_content)?;

    let mut texts = Vec::new();

    if let Some(body) = doc.descendants().find(|n| n.has_tag_name("body")) {
        // Iterate over <sec> tags within the <body> tag
        for sec in body.descendants().filter(|n| n.has_tag_name("sec")) {
            // Iterate over <p> tags within each <sec> tag
            for p in sec.descendants().filter(|n| n.has_tag_name("p")) {
                let mut text_content = String::new();
                for descendant in p.descendants() {
                    if descendant.is_text() {
                        text_content.push_str(descendant.text().unwrap_or(""));
                    }
                }
                if !text_content.is_empty() {
                    texts.push(text_content);
                }
            }
        }
    }
    
    Ok(texts)
}

fn extract_text_from_sec<P: AsRef<Path>>(file_path: P) -> Result<Vec<(String, Vec<String>)>, Box<dyn std::error::Error>> {
    let xml_content = fs::read_to_string(file_path)?;
    let doc = Document::parse(&xml_content)?;

    let mut sections = Vec::new();

    // Find the <body> tag
    if let Some(body) = doc.descendants().find(|n| n.has_tag_name("body")) {
        // Iterate over <sec> tags within the <body> tag
        for sec in body.descendants().filter(|n| n.has_tag_name("sec")) {
            // Extract the title text
            let title_text = sec.descendants().find(|n| n.has_tag_name("title"))
                .and_then(|n| n.text())
                .unwrap_or_default()
                .to_string();

            // Extract texts from <p> tags
            let mut p_texts = Vec::new();
            for p in sec.descendants().filter(|n| n.has_tag_name("p")) {
                let mut text_content = String::new();
                for descendant in p.descendants() {
                    if descendant.is_text() {
                        text_content.push_str(descendant.text().unwrap_or(""));
                    }
                }
                if !text_content.is_empty() {
                    p_texts.push(text_content);
                }
            }

            if !title_text.is_empty() || !p_texts.is_empty() {
                sections.push((title_text, p_texts));
            }
        }
    }

    Ok(sections)
}
