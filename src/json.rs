use std::fs;
use serde_json::Value;
use serde_json::json;
use std::path::Path;

use std::collections::BTreeMap;

use regex::Regex;

use crate::Args;
use clap::Parser;

use anyhow::{Context, Result};

// ===========================================================================

pub fn extract_text_from_json<P: AsRef<Path>>(file_path: P) -> Result<BTreeMap<String, String>> {
    let data = fs::read_to_string(file_path)?;
    let json: Value = serde_json::from_str(&data)?;

    //dbg!("{:?}", &json);

    // body_text is the COVID-19 data, the newer ones have passages/text etc.
    let body_text = json["body_text"].as_array().context("Invalid format for 'body_text'")?;
    
    let mut fulltext = BTreeMap::new(); // This one is sorted.
    let mut current_section = String::from("UNKNOWN");
    let mut section_number = 0usize;

    let remove_simpleref = Regex::new(r"\n\d{1,2}").unwrap();
    let remove_latex = Regex::new(r"(?s)\\documentclass.*?\\end\{document\}").unwrap(); // (?s) = multi-line
    let remove_figs = Regex::new(r"\(Fig(?:ure|\.)? \d+[a-z]?\)").unwrap();
    //let remove_figs1 = Regex::new(r"\(Fig\.?\s*ure?\s+\d+[a-z]?(?:,\s*[a-z])?\)").unwrap();
    //let remove_figs1 = Regex::new(r"\(Fig\.?\s*ure?\s+\d+(?:[a-z](?:,\s*[a-z])?)?\)").unwrap();
    ////let remove_parens = Regex::new(r"\([^)]*\)").unwrap(); // Takes too much...
    let remove_parens = Regex::new(r"\([^)]{1,10}\)").unwrap();
    let remove_square = Regex::new(r"\[\s*\d+\s*(,\s*\d+\s*)*\]").unwrap();
    //Regex::new(r"\[\d+(,\d+)*\]").unwrap(); //Regex::new(r"\[\d+\]").unwrap();
    
    let args = Args::parse();

    let mut sep = "";
    for entry in body_text {
        if let Some(text) = entry["section"].as_str() {
            let clean_text = String::from(text);
            // A new section (assuming they are in order)?
            // Prepend a number so they can be sorted later.
            if current_section != format!("{:02}:{}", section_number, clean_text.clone()) { 
                section_number += 1;
                sep = "";
                current_section = format!("{:02}:{}", section_number, clean_text.clone());
            }
        }
        // store as datat[section] += clean_text or something.
        if let Some(text) = entry["text"].as_str() {
            let mut clean_text = String::from(text);

            clean_text = remove_latex.replace_all(&clean_text, "").into_owned(); // always
            if args.remove {
                // Quick fix for "\n1" type of refs.
                clean_text = remove_simpleref.replace_all(&clean_text, "").into_owned();
                clean_text = remove_figs.replace_all(&clean_text, "").into_owned();
                //let clean_text = remove_figs1.replace_all(&clean_text, "");
                clean_text = remove_parens.replace_all(&clean_text, "").into_owned();
                clean_text = remove_square.replace_all(&clean_text, "").into_owned();
            }
            clean_text = sep.to_owned() + &clean_text;
            sep = " ";

            fulltext.entry(current_section.clone()).or_insert_with(String::new).push_str(&clean_text);
        }
    }
    
    Ok(fulltext)
}

// -------------------------------------------------------------------

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Infons {
    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Passage {
    offset: u32,
    infons: Infons,
    text: String,
    sentences: Vec<serde_json::Value>,
    annotations: Vec<serde_json::Value>,
    relations: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Document {
    id: String,
    infons: Infons,
    passages: Vec<Passage>,
    annotations: Vec<serde_json::Value>,
    relations: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Root {
    source: String,
    date: String,
    key: String,
    infons: Infons,
    documents: Vec<Document>,
}

pub fn extract_text_from_json_2<P: AsRef<Path>>(file_path: P) -> Result<BTreeMap<String, String>> {
    let data = fs::read_to_string(file_path)?;

    //let json: Value = serde_json::from_str(&data)?;

    let root: Root = serde_json::from_str(&data).expect("JSON was not well-formatted");

    for document in root.documents {
        for passage in document.passages {
            println!("{}", passage.text);
        }
    }

    
    let mut fulltext = BTreeMap::new(); // This one is sorted.
    let mut current_section = String::from("UNKNOWN");
    let mut section_number = 0usize;

    let remove_simpleref = Regex::new(r"\n\d{1,2}").unwrap();
    let remove_latex = Regex::new(r"(?s)\\documentclass.*?\\end\{document\}").unwrap(); // (?s) = multi-line
    let remove_figs = Regex::new(r"\(Fig(?:ure|\.)? \d+[a-z]?\)").unwrap();
    //let remove_figs1 = Regex::new(r"\(Fig\.?\s*ure?\s+\d+[a-z]?(?:,\s*[a-z])?\)").unwrap();
    //let remove_figs1 = Regex::new(r"\(Fig\.?\s*ure?\s+\d+(?:[a-z](?:,\s*[a-z])?)?\)").unwrap();
    ////let remove_parens = Regex::new(r"\([^)]*\)").unwrap(); // Takes too much...
    let remove_parens = Regex::new(r"\([^)]{1,10}\)").unwrap();
    let remove_square = Regex::new(r"\[\s*\d+\s*(,\s*\d+\s*)*\]").unwrap();
    //Regex::new(r"\[\d+(,\d+)*\]").unwrap(); //Regex::new(r"\[\d+\]").unwrap();
    
    let args = Args::parse();

    
    Ok(fulltext)
}

// ----------------------------------------------------------------------------


// Remove the "NN:" prefix from the String.
pub fn remove_section_no(section: &String) -> String {
    if section.len() > 3 {
        section.chars().skip(3).collect()
    } else {
        String::new()
    }
}

pub fn output_json(filename: &str, texts: BTreeMap<String, String>) {

    let mut sections = vec![];
    let args = Args::parse();

    for (section, text) in &texts {
        let sect = if args.sectionnames {
            json!({
                "section": remove_section_no(section),
                "text": text,
            })
        } else {
            json!({
                "text": text,
            })
        };
        //println!("JSON {}", sect);
        sections.push(sect);
    }

    // Create a top level struct.
    let doc = json!({
        "filename": filename,
        "sections": sections,
    });
    println!("{}", doc);
}

