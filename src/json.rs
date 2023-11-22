use std::fs;
use serde_json::Value;
use std::path::Path;

use std::collections::BTreeMap;

//extern crate regex;
use regex::Regex;

// ===========================================================================

pub fn extract_text_from_json<P: AsRef<Path>>(file_path: P) -> Result<BTreeMap<String, String>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(file_path)?;
    let json: Value = serde_json::from_str(&data)?;

    let body_text = json["body_text"].as_array().ok_or("Invalid format for 'body_text'")?;

    let mut fulltext = BTreeMap::new(); // This one is sorted.
    let mut current_section = String::from("UNKNOWN");
    let mut section_number = 0usize;

    let remove_simpleref = Regex::new(r"\n\d{1,2}").unwrap();
    let remove_latex = Regex::new(r"(?s)\\documentclass.*?\\end\{document\}").unwrap();
    // “ ”
    //  [1, 2]
    // (Golden, 2020) or (Zimmerman & Curtis, 2020)
    // (Fig. 3)
    
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
            let clean_text = String::from(text);

            // Quick fix for "\n1" type of refs.
            let clean_text = remove_simpleref.replace_all(&clean_text, "");
            let clean_text = remove_latex.replace_all(&clean_text, "");
            
            if false {
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

    //let _: () = fulltext; // BTreeMap<String, String>
    //println!("{:?}", fulltext);
    
    // Iterate over everything.
    /*for (section, text) in &fulltext {
        println!("{section}");
    }*/
    
    Ok(fulltext)
}

