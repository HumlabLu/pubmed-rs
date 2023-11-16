use std::fs;
use std::error::Error;
use serde_json::{Value, from_str};
use std::path::{Path, PathBuf};

use std::collections::BTreeMap;

// ===========================================================================

pub fn extract_text_from_json<P: AsRef<Path>>(file_path: P) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(file_path)?;
    let json: Value = serde_json::from_str(&data)?;

    let body_text = json["body_text"].as_array().ok_or("Invalid format for 'body_text'")?;

    let mut texts = Vec::new();
    //let mut fulltext = HashMap::new();
    let mut fulltext = BTreeMap::new(); // This one is sorted.
    let mut current_section = String::from("UNKNOWN");
    let mut section_number = 0usize;
    
    for entry in body_text {
        if let Some(text) = entry["section"].as_str() {
            let mut clean_text = String::from(text);
            if current_section != format!("{:02}:{}", section_number, clean_text.clone()) { // a new one (assuming in order)
                section_number += 1;
                current_section = format!("{:02}:{}", section_number, clean_text.clone());
            }
            
            //println!("{}", clean_text); // Return this too?
            /*if !fulltext.contains_key(&current_section) {
                fulltext.insert(
                    current_section.clone(),
                    clean_text
                );
            }*/
        }
        // store as datat[section] += clean_text or something.
        if let Some(text) = entry["text"].as_str() {
            let mut clean_text = String::from(text);
            
            fulltext.entry(current_section.clone()).or_insert_with(String::new).push_str(&clean_text);
                
            /*if let Some(cite_spans) = entry["cite_spans"].as_array() {
                for cite_span in cite_spans.iter().rev() {
                    if let (Some(start), Some(end)) = (cite_span["start"].as_i64(), cite_span["end"].as_i64()) {
                        clean_text.replace_range(start as usize..=end as usize, "");
                    }
                }
            }*/

            texts.push(clean_text);
        }
    }

    println!("{:?}", fulltext);
    // Iterate over everything.
    for (section, text) in &fulltext {
        println!("{section}");
    }
    
    Ok(texts)
}

