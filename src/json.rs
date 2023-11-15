use std::fs;
use std::error::Error;
use serde_json::{Value, from_str};
use std::path::{Path, PathBuf};

// ===========================================================================

pub fn extract_text_from_json<P: AsRef<Path>>(file_path: P) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(file_path)?;
    let json: Value = serde_json::from_str(&data)?;

    let body_text = json["body_text"].as_array().ok_or("Invalid format for 'body_text'")?;

    let mut texts = Vec::new();

    for entry in body_text {
        if let Some(text) = entry["section"].as_str() {
            let mut clean_text = String::from(text);
            //println!("{}", clean_text); // Return this too?
        }
        if let Some(text) = entry["text"].as_str() {
            let mut clean_text = String::from(text);

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

    Ok(texts)
}

