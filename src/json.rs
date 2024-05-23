use std::fs;
use serde_json::Value;
use serde::{Deserialize, Serialize};

use std::path::Path;

use std::collections::BTreeMap;
use std::collections::HashMap;

use regex::Regex;

use crate::Args;
use clap::Parser;

use anyhow::{Context, Result};

// ===========================================================================

pub fn _extract_text_from_json<P: AsRef<Path>>(file_path: P) -> Result<BTreeMap<String, String>> {
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
    
    let _args = Args::parse();

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
            if true { // true was args.remove
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


#[derive(Debug, Deserialize, Serialize)]
struct Root {
    source: String,
    date: String,
    key: String,
    infons: HashMap<String, Option<String>>,
    documents: Vec<Document>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Document {
    id: String,
    infons: HashMap<String, Option<String>>,
    passages: Vec<Passage>,
    annotations: Vec<Annotation>,
    relations: Vec<Relation>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Passage {
    offset: u32,
    infons: HashMap<String, Option<String>>,
    text: String,
    sentences: Vec<Sentence>,
    annotations: Vec<Annotation>,
    relations: Vec<Relation>,
}


#[derive(Debug, Deserialize, Serialize)]
struct Sentence {
    // Define fields based on the structure of sentences in your JSON
    // For example, if sentences contain only text:
    text: String
}

#[derive(Debug, Deserialize, Serialize)]
struct Annotation {
    // Define fields based on the structure of annotations in your JSON
    // For example, if annotations contain an id and type:
    // id: String,
    par_type: String
}

#[derive(Debug, Deserialize, Serialize)]
struct Relation {
    // Define fields based on the structure of relations in your JSON
    // For example, if relations contain an id and type:
    // id: String,
    par_type: String
}

/*
    Output JSON.
*/
#[derive(Deserialize, Serialize, Debug)]
pub struct OutputParagraph {
    par_type: String,
    text: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OutputArticle {
    pub paragraphs: Vec<OutputParagraph>,
    pub abbreviations: HashMap<String, String>,
}


pub fn extract_json_from_json<P: AsRef<Path>>(file_path: P) -> Result<Value> {
    let data = fs::read_to_string(file_path)?;

    //let json: Value = serde_json::from_str(&data)?;
    let root: Root = serde_json::from_str(&data)? ;//.expect("JSON was not well-formatted");
    //dbg!("{:?}", &root);

    // Test output JSON
    let mut od = OutputArticle { // OutputDocument?
        paragraphs: vec![],
        abbreviations: HashMap::new(),
    };
    
    for document in root.documents {
        //println!("{}", document.id);
        
        let mut abbr: Option<String> = None;

        for passage in document.passages {
            //dbg!("{:?}", &passage);
            let section_type = &passage.infons["section_type"].clone().unwrap(); // clone().unwrap because Option<...>
            let par_type = &passage.infons["type"].clone().unwrap();  // because Option<...>
            if (section_type == "REF")
                || (section_type == "FIG")
                || (section_type == "TABLE")
                || (section_type == "APPENDIX")
                || (section_type == "COMP_INT")
                || (section_type == "METHODS")
                || (section_type == "AUTH_CONT")
                || (section_type == "ACK_FUND")
                || (section_type == "SUPPL")
                || (section_type == "REVIEW_INFO") {
                    continue;
                }
            // Alternating abbreviation-meaning.
            if (section_type == "ABBR") && (par_type == "paragraph") {
                if abbr.is_none() { // Or None/Some(abbr)?
                    //println!("ABBR {}\t", passage.text);
                    if passage.text.len() < 10 {
                        //if !passage.text.contains(char::is_whitespace) {
                        abbr = Some(passage.text);
                    }
                } else {
                    //println!("{}", passage.text);
                    od.abbreviations.insert(abbr.clone().unwrap(), passage.text);
                    abbr = None;
                }
                continue;
            }
            //println!("{:?}", passage.infons);
            if par_type == "paragraph" || par_type == "abstract" {
                //println!("{} {}\n", section_type, passage.text);

                // Create a JSON paragraph.
                let op = OutputParagraph {
                    par_type: section_type.to_string(),
                    text: passage.text
                };
                //let js = serde_json::to_value(&op).unwrap();
                //dbg!("{}", js);
                od.paragraphs.push(op);
            }
        } // passages
    }
    
    let _remove_simpleref = Regex::new(r"\n\d{1,2}").unwrap();
    let _remove_latex = Regex::new(r"(?s)\\documentclass.*?\\end\{document\}").unwrap(); // (?s) = multi-line
    let _remove_figs = Regex::new(r"\(Fig(?:ure|\.)? \d+[a-z]?\)").unwrap();
    //let remove_figs1 = Regex::new(r"\(Fig\.?\s*ure?\s+\d+[a-z]?(?:,\s*[a-z])?\)").unwrap();
    //let remove_figs1 = Regex::new(r"\(Fig\.?\s*ure?\s+\d+(?:[a-z](?:,\s*[a-z])?)?\)").unwrap();
    ////let remove_parens = Regex::new(r"\([^)]*\)").unwrap(); // Takes too much...
    let _remove_parens = Regex::new(r"\([^)]{1,10}\)").unwrap();
    let _remove_square = Regex::new(r"\[\s*\d+\s*(,\s*\d+\s*)*\]").unwrap();
    //Regex::new(r"\[\d+(,\d+)*\]").unwrap(); //Regex::new(r"\[\d+\]").unwrap();

    let js = serde_json::to_value(&od).unwrap();
    Ok(js)
}

// ----------------------------------------------------------------------------


// Remove the "NN:" prefix from the String.
pub fn _remove_section_no(section: &String) -> String {
    if section.len() > 3 {
        section.chars().skip(3).collect()
    } else {
        String::new()
    }
}

pub fn output_json(_filename: &str, texts: Value) {
    println!("{}", serde_json::to_string_pretty(&texts).unwrap());
}

