use std::fs;
use serde_json::Value;
use std::path::Path;

use std::collections::BTreeMap;

use regex::Regex;

use crate::Args;
use clap::Parser;

use anyhow::{Context, Result};

// ===========================================================================

//pub fn extract_text_from_json<P: AsRef<Path>>(file_path: P) -> Result<BTreeMap<String, String>, Box<dyn std::error::Error>> {
pub fn extract_text_from_json<P: AsRef<Path>>(file_path: P) -> Result<BTreeMap<String, String>> {
    let data = fs::read_to_string(file_path)?;
    let json: Value = serde_json::from_str(&data)?;

    let body_text = json["body_text"].as_array().context("Invalid format for 'body_text'")?;

    let mut fulltext = BTreeMap::new(); // This one is sorted.
    let mut current_section = String::from("UNKNOWN");
    let mut section_number = 0usize;

    let remove_simpleref = Regex::new(r"\n\d{1,2}").unwrap();
    let remove_latex = Regex::new(r"(?s)\\documentclass.*?\\end\{document\}").unwrap(); // (?s) = multi-line
    let remove_figs = Regex::new(r"\(Fig(?:ure|\.)? \d+[a-z]?\)").unwrap();
    //let remove_figs1 = Regex::new(r"\(Fig\.?\s*ure?\s+\d+[a-z]?(?:,\s*[a-z])?\)").unwrap();
    //let remove_figs1 = Regex::new(r"\(Fig\.?\s*ure?\s+\d+(?:[a-z](?:,\s*[a-z])?)?\)").unwrap();
    let remove_parens = Regex::new(r"\([^)]*\)").unwrap();
    let remove_square = Regex::new(r"\[\d+\]").unwrap();
    
    let args = Args::parse();
    
    for entry in body_text {
        if let Some(text) = entry["section"].as_str() {
            let clean_text = String::from(text);
            // A new one (assuming they are in order)...
            if current_section != format!("{:02}:{}", section_number, clean_text.clone()) { 
                section_number += 1;
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
            
            if false { // Didn't really work as expected...
                if let Some(cite_spans) = entry["cite_spans"].as_array() {
                    for cite_span in cite_spans.iter().rev() {
                        println!("{:?}", cite_span);
                        /*if let (Some(start), Some(end)) = (cite_span["start"].as_i64(), cite_span["end"].as_i64()) {
                            // Does utf-8 give wrong spans?
                            // We cannot run this after the regexen.
                            //clean_text.replace_range(start as usize..=end as usize, "");
                        }*/
                    }
                }
            }
            
            fulltext.entry(current_section.clone()).or_insert_with(String::new).push_str(&clean_text);             
        }
    }
    
    Ok(fulltext)
}

