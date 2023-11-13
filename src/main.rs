use std::fs::File;
use std::io::BufReader;

use std::fs;
use std::path::{Path, PathBuf};
use std::io;

use log::debug;
use log::error;
use log::info;
use log::warn;

use clap::Parser;

use roxmltree::Document;

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
}

fn get_files_in_directory<P: AsRef<Path>>(path: P) -> io::Result<Vec<PathBuf>> {
    let mut file_paths = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "xml" {
                    file_paths.push(path);
                }
            }
        }
    }

    Ok(file_paths)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    /*debug!("Mary has a little lamb");
    error!("{}", "Its fleece was white as snow");
    info!("{:?}", "And every where that Mary went");
    warn!("{:#?}", "The lamb was sure to go");*/

    let args = Args::parse();

    // Check if dirname is not none first.
    if args.dirname.is_some() {
        match get_files_in_directory(args.dirname.unwrap()) {
            Ok(files) => {
                for file in files {
                    println!("{:?}", file);

                    match extract_text_from_sec(file) {
                        Ok(sections) => {
                            for (title, texts) in sections {
                                println!("Title: {}", title);
                                for text in texts {
                                    println!("Text: {}", text);
                                }
                                println!("---"); // Separator for different sections
                            }
                        }
                        Err(e) => eprintln!("Error: {}", e),
                    }

                }
            }
            Err(e) => eprintln!("Failed to read directory: {}", e),
        }
    }

    /*match extract_text_from_p_tags_in_sec("./PMC10000424.fmt.xml") {
        Ok(texts) => {
            for text in texts {
                println!("{}", text);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
        }*/
   
    
    if args.filename.is_some() {
        let path_name = args.filename.unwrap();
        info!("FILE {path_name}");
        match extract_text_from_sec(path_name) {
            Ok(sections) => {
                for (title, texts) in sections {
                    println!("Title: {}", title);
                    for text in texts {
                        println!("Text: {}", text);
                    }
                    println!("---"); // Separator for different sections
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    
    Ok(())
}

// ================================================================
// roxmltree
// ================================================================

fn extract_text_from_sec_tags(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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

fn extract_text_from_p_tags_in_sec(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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
