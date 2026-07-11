use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use quick_xml::Reader;
use quick_xml::events::Event;
use crate::utils::clinical_sections::Section;
use crate::utils::common_structs::{BaseIdentifier};

#[derive(Debug, PartialEq)]
enum ParseState {
    Root,
    InSection,
    InEntry,
    InAct,
}

pub fn problem_section(file_path_str: &str) -> Section {
    let file_path = Path::new(file_path_str);
    let file = File::open(file_path).expect("Unable to open check if file or path exists!");
    let reader = BufReader::new(file);
    let mut xml = Reader::from_reader(reader);
    let mut buf = Vec::new();
    let mut state = ParseState::Root;
    let mut section = Section {
        template_ids: Vec::new(),
        code: None,
        title: None,
        text: None,
        entries: Vec::new(),
    };

    loop {
        match xml.read_event_into(&mut buf) {
            // for error handling
            Err(e) => panic!("Error occured when parsing for {:?} where byte position = {}, error position = {}", e, xml.buffer_position(), xml.error_position()),
            // works for opening tag for example <act>
            Ok(Event::Start(e)) => {
                // println!("event start name {:?}", e.name());
                match e.name().as_ref() {
                    b"section" => state = ParseState::InSection,
                    b"entry" => state = ParseState::InEntry,
                    b"act" => state = ParseState::InAct,
                    _ => {}
                }
            }
            // works for closing tag for example </act>
            Ok(Event::End(e)) => {
                // println!("event end name {:?}", e.name());
                match e.name().as_ref() {
                    b"section" => state = ParseState::Root,
                    b"entry" => state = ParseState::InSection,
                    b"act" => state = ParseState::InEntry,
                    _ => {}
                }
            }
            // works for self-closing tag for example <templateId/>
            Ok(Event::Empty(e)) => {
                if state == ParseState::InSection {
                    match e.name().as_ref() {
                        b"templateId" => {
                            let root = e.try_get_attribute(b"root")
                                .unwrap()
                                .map(|a| String::from_utf8(a.value.to_vec()).unwrap());
                            println!("InSection > templateId root => {:?}", root);
                            let extension = e.try_get_attribute(b"extension")
                                .unwrap()
                                .map(|a| String::from_utf8(a.value.to_vec()).unwrap());
                            println!("InSection > templateId extension => {:?}", extension);
                            let template_id = BaseIdentifier {
                                root: root,
                                extension: extension,
                            };
                            section.template_ids.push(template_id);
                        }
                        b"code" => {
                            let code = e.try_get_attribute(b"code");
                            println!("code => InSection {:?}", code);
                            let code_system = e.try_get_attribute(b"codeSystem");
                            println!("codeSystem => InSection {:?}", code_system);
                            let display_name = e.try_get_attribute(b"displayName");
                            println!("displayName => InSection {:?}", display_name);
                            let code_system_name = e.try_get_attribute(b"codeSystemName");
                            println!("codeSystemName => InSection {:?}", code_system_name);
                        }
                        _ => {}
                    }
                }
                // if state == ParseState::InEntry {
                //     match e.name().as_ref() {
                //         b"act" => {
                //             let class_code = e.try_get_attribute(b"classCode");
                //             println!("classCode => InEntry {:?} ", class_code);
                //             let mood_code = e.try_get_attribute(b"moodCode");
                //             println!("moodCode => InEntry {:?} ", mood_code);
                //         }
                //         _ => {}
                //     }
                // }
            }
            Ok(Event::Eof) => break,
            // skip other events
            _ => (),
        }
        buf.clear();
    }
    return section;
    // todo!()
}