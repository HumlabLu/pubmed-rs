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
    
    for entry in body_text {
        if let Some(text) = entry["section"].as_str() {
            let clean_text = String::from(text);
            // A new section (assuming they are in order)?
            // Prepend a number so they can be sorted later.
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
                        
            fulltext.entry(current_section.clone()).or_insert_with(String::new).push_str(&clean_text);             
        }
    }
    
    Ok(fulltext)
}

// Remove the "NN:" prefix from the String.
pub fn remove_section_no(section: &String) -> String {
    if section.len() > 3 {
        section.chars().skip(3).collect()
    } else {
        String::new()
    }
}

/*
{
  "2303949": {
    "title": "Excursion of the flexor digitorum profundus tendon: ...",
    "abstract": "The most common problem following ...",
    "mesh_terms": "D000818:Animals; D004285:Dogs; D005385:Fingers; D006801:Humans; D007596:Joints; D008662:Metacarpophalangeal Joint; D008663:Metac
arpus; D009068:Movement; D013710:Tendons",
    "pubdate": "1990-03",
    "chemical_list": ""
    },


72803	{'text': 'A combined familial study of multiple sclerosis (MS) in England and in the Rostock area of the GDR using the macrophage electrophoretic mobility (MEM)-LAD test embracing 132 relatives has revealed a closely similar pattern of distribution of "anomalous" LAD (Linoleic Acid Depression) values in relatives (77% type of reaction) to that originally reported in the British study.', 'entities': {'disease': ['multiple sclerosis', 'ms', 'acid depression'], 'chemical': ['linoleic acid']}, 'entity_spans': {'disease': [[29, 47], [49, 51], [268, 283]], 'chemical': [[259, 272]]}}
*/
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

