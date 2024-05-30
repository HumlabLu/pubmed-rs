use std::fs;
use serde::{Deserialize, Serialize};

use std::path::Path;

use std::collections::BTreeSet;
use std::collections::HashMap;

use regex::Regex;

use crate::Args;
use clap::Parser;

use anyhow::{Result};
use crate::error;

// ===========================================================================

#[derive(Debug, Deserialize, Serialize)]
struct Root {
    source: String,
    date: String,
    //key: String,
    infons: HashMap<String, Option<String>>,
    documents: Vec<Document>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Document {
    id: String,
    infons: HashMap<String, Option<String>>,
    passages: Vec<Passage>,
    //annotations: Vec<Annotation>,
    //relations: Vec<Relation>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Passage {
    offset: u32,
    infons: HashMap<String, Option<String>>,
    text: String,
    //sentences: Vec<Sentence>,
    //annotations: Vec<Annotation>,
    //relations: Vec<Relation>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Sentence {
    text: String
}

#[derive(Debug, Deserialize, Serialize)]
struct Annotation {
    par_type: String
}

#[derive(Debug, Deserialize, Serialize)]
struct Relation {
    par_type: String
}

/*
    Output JSON.
*/
#[derive(Deserialize, Serialize, Debug)]
pub struct OutputParagraph {
    pub par_type: String,
    pub text: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OutputArticle {
    pub paragraphs: Vec<OutputParagraph>,
    pub abbreviations: HashMap<String, String>,
    pub year: String,
    pub pmid: String,
    pub title: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OutputChunk {
    pub articles: HashMap<String, OutputArticle>,
}


// The extra filename is for printing error info. Our signature doesn't
// allow printing of "Path", and the directory version sends Paths
// this way. This should be fixed!
pub fn extract_json_from_json<P: AsRef<Path>>(file_path: P, filename: &str, allowed: &BTreeSet<String>) -> Result<OutputArticle> {
    let data = fs::read_to_string(file_path)?;
    
    //let json: Value = serde_json::from_str(&data)?;
    let root: Root = serde_json::from_str(&data)? ;//.expect("JSON was not well-formatted");
    //dbg!("{:?}", &root);

    // Test output JSON
    let mut od = OutputArticle { // OutputDocument?
        paragraphs: vec![],
        abbreviations: HashMap::new(),
        year: "UNK".to_string(),
        pmid: "UNK".to_string(),
        title: "UNK".to_string(),
    };

    let args = Args::parse();
    
    for document in root.documents {
        //println!("{}", document.id);
        
        let mut abbr: Option<String> = None;

        for passage in document.passages {
            //dbg!("{:?}", &passage);

            // Some documents don't have section types?
            if passage.infons.contains_key("section_type") {
                
                let section_type = &passage.infons["section_type"].clone().unwrap(); // clone().unwrap because Option<...>                
                let par_type = &passage.infons["type"].clone().unwrap();  // because Option<...>
                
                if par_type == "front" && passage.offset == 0 {
                    if passage.infons.contains_key("year") {
                        od.year = passage.infons["year"].clone().unwrap();
                     }  
                    if passage.infons.contains_key("article-id_pmc") {
                        od.pmid = passage.infons["article-id_pmc"].clone().unwrap();
                    }
                    od.title = passage.text.clone();
                }

                if allowed.is_empty() {
                    if (section_type == "REF")
                        || (section_type == "FIG")
                        || (section_type == "TABLE")
                        || (section_type == "APPENDIX")
                        || (section_type == "COMP_INT")
                        || (section_type == "CASE")
                        //|| (section_type == "METHODS") // Yes, no, maybe?
                        || (section_type == "AUTH_CONT")
                        || (section_type == "ACK_FUND")
                        || (section_type == "SUPPL")
                        || (section_type == "REVIEW_INFO") {
                            continue;
                        }
                } else { // allowed is not empty
                    if section_type != "ABBR" { // but ABBR goes through anyway
                        if ! allowed.contains(section_type) {
                            continue;
                        }
                    }
                }
                
                // Alternating abbreviation-meaning.
                if (section_type == "ABBR") && (par_type == "paragraph") {
                    if abbr.is_none() { 
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

                if par_type == "paragraph" || par_type == "abstract" {

                    if args.sentences == false {
                        // Create a JSON paragraph.
                        let op = OutputParagraph {
                            par_type: section_type.to_string(),
                            text: passage.text.clone()
                        };
                        //let js = serde_json::to_value(&op).unwrap();
                        //dbg!("{}", js);
                        od.paragraphs.push(op);
                    } else {
                        for s in cutters::cut(&passage.text, cutters::Language::English) {
                            let op = OutputParagraph {
                                par_type: section_type.to_string(),
                                text: s.str.to_string()
                            };
                            od.paragraphs.push(op);
                        }
                    }
                }
            } else { // has no section_type
                error!("{}: passage has no section_type.", filename);
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

    Ok(od)
}

pub fn output_json(_filename: &str, texts: OutputArticle) {
    println!("{}", serde_json::to_string_pretty(&texts).unwrap());
}

