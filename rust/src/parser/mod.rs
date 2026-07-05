use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use quick_xml::Reader;
use quick_xml::events::Event;
use crate::utils::clinical_sections::Section;

pub fn problem_section(file_path_str: &str) -> Section {
    let file_path = Path::new(file_path_str);
    let file = File::open(file_path).expect("Unable to open check if file or path exists!");
    let reader = BufReader::new(file);
    let mut xml = Reader::from_reader(reader);
    let mut buf = Vec::new();

    loop {
        match xml.read_event_into(&mut buf) {
            // for error handling
            Err(e) => panic!("Error occured when parsing for {:?} where byte position = {}, error position = {}", e, xml.buffer_position(), xml.error_position()),
            // works for opening tag for example <act>
            Ok(Event::Start(e)) => {
                // println!("event start name {:?}", e.name());
            }
            // works for closing tag for example </act>
            Ok(Event::End(e)) => {
                // println!("event end name {:?}", e.name());
            }
            // works for self-closing tag for example <templateId/>
            Ok(Event::Empty(e)) => {
                // println!("event empty name {:?}", e.name());
            }
            Ok(Event::Eof) => break,
            // skip other events
            _ => (),
        }
        buf.clear();
    }
    Section {
        template_ids: Vec::new(),
        code: None,
        title: None,
        text: None,
        entries: Vec::new(),
    }

    // todo!()
}