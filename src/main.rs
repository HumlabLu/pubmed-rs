use std::fs::File;
use std::io::BufReader;
use xml::common::Position;
use xml::reader::{ParserConfig, XmlEvent, EventReader};

use std::fs;

fn main() -> Result<(), quick_xml::Error> {
    let paths = fs::read_dir("/Users/pberck/Downloads/PMC010xxxxxx/").unwrap();
    for path in paths {
        let path_name = path.unwrap().path().display().to_string();
        println!("FILE {path_name}");
        let file = File::open(path_name).unwrap();
        
        let mut reader = ParserConfig::default()
            .ignore_root_level_whitespace(false)
            .create_reader(BufReader::new(file));
        
        reader = find_tag(reader, "article-title"); 
        reader = loop_until_end_of(reader, "article-title"); // only finds one...
    }

    //xmlrs(String::from("/Users/pberck/Downloads/PMC010xxxxxx/PMC10254423.xml"));
    //xmlrs(String::from("/Users/pberck/Downloads/PMC010xxxxxx/PMC10254128.xml"));
    xmlrs(String::from("/Users/pberck/Downloads/PMC010xxxxxx/PMC10000424.xml"));

    Ok(())
}

// ================================================================
// xml-rs code example
// ================================================================

// We need a "find" as well... find abstract, return text, or something.

// An extract_X_until_Y function?

// A <<sec> contains multiple <p> with text. We need a func to extract it.
/*
  Find returns with the reader on the tag.
*/
fn find_tag(mut reader: EventReader<BufReader<File>>, tag: &str) -> EventReader<BufReader<File>> {
    println!("find({tag})");
    loop {
        match reader.next() {
            Ok(e) => {
                //print!("{}\t", reader.position());
                match e {
                    XmlEvent::StartDocument {
                        version, encoding, ..
                    } => {
                        println!("StartDocument({version}, {encoding})")
                    }
                    XmlEvent::EndDocument => {
                        println!("EndDocument");
                        break;
                    }
                    XmlEvent::ProcessingInstruction { name, data } => {
                        println!(
                            "ProcessingInstruction({name}={:?})",
                            data.as_deref().unwrap_or_default()
                        )
                    }
                    XmlEvent::StartElement {
                        name, attributes, ..
                    } => {
                        if attributes.is_empty() {
                            if name.local_name == tag {
                                println!("FOUND {tag}");
                                return reader;
                            }
                        } else { // We're not using the attributes, could be combined.
                            let attrs: Vec<_> = attributes
                                .iter()
                                .map(|a| format!("{}={:?}", &a.name, a.value))
                                .collect();
                            ////println!("StartElement({name} [{}])", attrs.join(", "));
                            if name.local_name == tag {
                                println!("FOUND {tag}");
                                return reader;
                            }
                        } // else
                    }, // StartElement
                    _ => {
                        //println!("Searching for {tag}.")
                    },
                } // match e
            }, // Ok
            Err(e) => {
                eprintln!("Error at {}: {e}", reader.position());
                break;
            } // Err
        } // reader.next
    } // loop
    
    reader
}

// Consume some, and return the reader. Consumes until the last "tag" has been
// consumed. It returns when we move a level up again, so it is really
// all sub-tags.
fn loop_until_end_of(mut reader: EventReader<BufReader<File>>, tag: &str) -> EventReader<BufReader<File>> {
    println!("loop_until_end_of({tag})");

    let mut depth = 0;
    let mut current_tag = String::from(tag);// = tag;
    
    // We are in a certain tag, loop until we find a closing
    // tag on the same depth.
    loop {
        match reader.next() {
            Ok(e) => {
                //print!("{}\t", reader.position());
                match e {
                    XmlEvent::EndElement { name } => {
                        println!("EndElement({name}, at {depth})");
                        if depth == 0 && name.local_name == tag {
                            println!("End of {tag}.");
                            return reader
                        }
                        depth -= 1;
                        if depth < 0 {
                            return reader;
                        }
                    },
                    XmlEvent::StartElement {
                        name, attributes, ..
                    } => {
                        // Maybe have another parameter to only get a specific sub tag?
                        depth += 1;
                        current_tag = name.local_name.clone();
                    },
                    XmlEvent::EndDocument => { // this could happen?
                        println!("EndDocument");
                        break;
                    },
                    XmlEvent::Characters(data) => {
                        println!(r#"loop({current_tag}) {}"#, data.escape_debug()) // Return/save this also?
                    },
                    _ => {println!("waiting")},
                } // match e
            }, // OK
            Err(e) => {
                eprintln!("Error at {}: {e}", reader.position());
                break;
            } // Err
        } // reader-next()
    } // loop

    reader
}

fn xmlrs(file_path: String) {
    //let file_path = String::from("/Users/pberck/Downloads/PMC010xxxxxx/PMC10000166.xml");
    /*let file_path = std::env::args_os()
    .nth(1)
    .expect("Please specify a path to an XML file");*/

    println!("FILE {file_path}");
    let file = File::open(file_path.clone()).unwrap();

    let mut reader = ParserConfig::default()
        .ignore_root_level_whitespace(false)
        .create_reader(BufReader::new(file));

    // All <sec> text
    loop {
        match reader.next() { // peek ?
            Ok(e) => {
                match e {
                    XmlEvent::EndDocument => {
                        println!("EndDocument");
                        break;
                    },
                    XmlEvent::StartElement {
                        name, attributes, ..
                    } => {
                        if name.local_name == "sec" {
                            reader = loop_until_end_of(reader, "p"); // All <p> under <sec>
                        }
                    },
                    _ => {}
                }
            },
            Err(e) => {
                eprintln!("Error at {}: {e}", reader.position());
                break;
            }
        }
    }
    return;
    
    reader = find_tag(reader, "article-id"); 
    reader = loop_until_end_of(reader, "article-id"); // only finds one...

    reader = find_tag(reader, "abstract"); // we really want the <astract>...</abstract> sub-tree.
    reader = loop_until_end_of(reader, "abstract"); // Problem is, if we have a <italic> in the text...

    reader = find_tag(reader, "kwd-group"); // we really want the <astract>...</abstract> sub-tree.
    reader = loop_until_end_of(reader, "kwd-group"); // Problem is, if we have a <italic> in the text...

    //return;
    
    loop {
        match reader.next() {
            Ok(e) => {
                //print!("{}\t", reader.position());

                match e {
                    XmlEvent::StartDocument {
                        version, encoding, ..
                    } => {
                        println!("StartDocument({version}, {encoding})")
                    }
                    XmlEvent::EndDocument => {
                        println!("EndDocument");
                        break;
                    }
                    XmlEvent::ProcessingInstruction { name, data } => {
                        println!(
                            "ProcessingInstruction({name}={:?})",
                            data.as_deref().unwrap_or_default()
                        )
                    }
                    XmlEvent::StartElement {
                        name, attributes, ..
                    } => {
                        if attributes.is_empty() {
                            ////println!("StartElement({name})");
                            if name.local_name == "title-group" {

                                reader = loop_until_end_of(reader, "article-title");
                                
                                let maybe_title = reader.next();
                                let maybe_title = reader.next();
                                //XmlEvent::Characters(data) => {
                                match maybe_title {
                                    Ok(e) => {
                                        match e {
                                            XmlEvent::Characters(data) => {
                                                println!(r#"a-t {}"#, data.escape_debug())
                                            },
                                            _ => {println!("")},//todo!(), // Ignore the other XmlEvents.
                                        }
                                    },// Ok
                                    Err(e) => {
                                        eprintln!("Error at {}: {e}", reader.position());
                                        break;
                                    },
                                } //Match maybe_title
                            } // arcticle-title
                            if name.local_name == "article-meta" {
                                reader = loop_until_end_of(reader, "article-id");                                
                            }
                        } else {
                            let attrs: Vec<_> = attributes
                                .iter()
                                .map(|a| format!("{}={:?}", &a.name, a.value))
                                .collect();
                            ////println!("StartElement({name} [{}])", attrs.join(", "));

                            if name.local_name == "sec" {
                                print!("StartElement({name} [{}]): ", attrs.join(", "));
                                let maybe_title = reader.next().unwrap();
                                //println!("{:?}", maybe_title);
                                let maybe_title = reader.next();
                                //XmlEvent::Characters(data) => {
                                match maybe_title {
                                    Ok(e) => {
                                        match e {
                                            XmlEvent::Characters(data) => {
                                                println!(r#"{}"#, data.escape_debug())
                                            },
                                            _ => {println!("")},//todo!(), // Ignore the other XmlEvents.
                                        }
                                    },// Ok
                                    Err(e) => {
                                        eprintln!("Error at {}: {e}", reader.position());
                                        break;
                                    },
                                } //match maybe_title
                            } // local_name
                        } // else
                    } // StartElement
                    XmlEvent::EndElement { name } => {
                        //println!("EndElement({name})")
                    }
                    XmlEvent::Comment(data) => {
                        ////println!(r#"Comment("{}")"#, data.escape_debug())
                    }
                    XmlEvent::CData(data) => println!(r#"CData("{}")"#, data.escape_debug()),
                    XmlEvent::Characters(data) => {
                        ////println!(r#"Characters("{}")"#, data.escape_debug())
                    }
                    XmlEvent::Whitespace(data) => {
                        ////println!(r#"Whitespace("{}")"#, data.escape_debug())
                    }
                }
            }
            Err(e) => {
                eprintln!("Error at {}: {e}", reader.position());
                break;
            }
        }
    }
}
