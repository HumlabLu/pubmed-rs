use entrez_rs::errors::Error;
use entrez_rs::eutils::{EFetch, ESearch, Eutils, DB};
use entrez_rs::parser::esearch::ESearchResult;
use entrez_rs::parser::pubmed::PubmedArticleSet;

use quick_xml::events::{BytesStart, Event};
use quick_xml::name::QName;

use std::fs::File;
use std::io::BufReader;
use xml::common::Position;
use xml::reader::{ParserConfig, XmlEvent};

use std::fs;

fn entrez() -> Result<(), Error> {
    let xml = ESearch::new(DB::Pubmed, "cell death").run()?;
    
    let parsed = ESearchResult::read(&xml);

    println!("{:#?}", &parsed?.id_list.ids);

    let pm_xml = EFetch::new(DB::Pubmed, vec!["33246200", "33243171"]).run()?;

    let pm_parsed = PubmedArticleSet::read(&pm_xml);

    println!("{:?}", pm_parsed?.articles);

    Ok(())
}

fn main() -> Result<(), quick_xml::Error> {
    use quick_xml::events::Event;
    use quick_xml::reader::Reader;
    
    let mut reader = Reader::from_file("/Users/pberck/Downloads/PMC010xxxxxx/PMC10000066.xml")?;
    reader.trim_text(true);

    let mut count = 0;
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let name = e.name();
                let name = reader.decoder().decode(name.as_ref())?;
                println!("read start event {:?}", name.as_ref());
                count += 1;
            }
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
    }

    println!("read {} start events in total", count);

    //Ok(())

    reader = Reader::from_file("/Users/pberck/Downloads/PMC010xxxxxx/PMC10000066.xml")?;
    let mut count = 0;
    let mut txt = Vec::new();
    let mut buf = Vec::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        // NOTE: this is the generic case when we don't know about the input BufRead.
        // when the input is a &str or a &[u8], we don't actually need to use another
        // buffer, we could directly call `reader.read_event()`
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            // exits the loop when reaching end of file
            Ok(Event::Eof) => break,

            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"italic" => println!(
                    "attributes values: {:?}",
                    e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()
                ),
                b"title" => {
                    count += 1;
                    /*
                            println!("{:?}", e);
                            let mut element_buf = Vec::new();
                            let event = reader.read_event_into(&mut element_buf)?;
                            println!("an event {:?}", event);
                            if let Event::Start(ref e) = event {
                                let name = e.name();
                                let mut tmp_buf = Vec::new();
                                let text_content = reader.read_to_end_into(e.name(), &mut tmp_buf).unwrap();
                                println!("{:?}", text_content);
                            } else {
                                let event_string = format!("{:?}", event);
                                break; //Err(quick_xml::Error::UnexpectedToken(event_string))
                        }
                    */
                }
                _ => (),
            },
            Ok(Event::Text(e)) => {
                //println!("{}", e.unescape().unwrap());
                txt.push(e.unescape().unwrap().into_owned());
            }

            // There are several other `Event`s we do not consider here
            _ => (),
        }
        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }
    println!("read {} count", count);
    println!("{:?}", txt);

    //Ok(())
    //entrez();

    let paths = fs::read_dir("/Users/pberck/Downloads/PMC010xxxxxx/").unwrap();
    for path in paths {
        println!("--------Name: {}", path.as_ref().unwrap().path().display());
        //xmlrs(path.unwrap().path().display().to_string());
    }

    xmlrs(String::from("/Users/pberck/Downloads/PMC010xxxxxx/PMC10254423.xml"));
    xmlrs(String::from("/Users/pberck/Downloads/PMC010xxxxxx/PMC10254128.xml"));

    Ok(())
}

// ================================================================
// xml-rs code example
// ================================================================

fn xmlrs(file_path: String) {
    //let file_path = String::from("/Users/pberck/Downloads/PMC010xxxxxx/PMC10000166.xml");
    /*let file_path = std::env::args_os()
    .nth(1)
    .expect("Please specify a path to an XML file");*/
    let file = File::open(file_path).unwrap();

    let mut reader = ParserConfig::default()
        .ignore_root_level_whitespace(false)
        .create_reader(BufReader::new(file));

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
                                // We should process until we get an EndElement event...
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
                                } //match maybe_title
                            } // arcticle-title
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
