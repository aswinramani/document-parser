use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use quick_xml::Reader;
use quick_xml::events::Event;
use crate::utils::clinical_sections::{Section, Entry,ClinicalStatement, EntryAct, ActBody};
use crate::utils::common_structs::{BaseIdentifier, Code};

#[derive(Debug, PartialEq)]
enum ParseState {
    Root,
    InSection,
    InEntry,
    InAct,
    InEntryRelationship,
    InAuthor,
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
    let mut current_entry: Option<Entry> = None;
    let mut current_act: Option<EntryAct> = None;
    loop {
        match xml.read_event_into(&mut buf) {
            // for error handling
            Err(e) => panic!("Error occured when parsing for {:?} where byte position = {}, error position = {}", e, xml.buffer_position(), xml.error_position()),
            // works for opening tag for example <act>
            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"section" => state = ParseState::InSection,
                    b"entry" => {
                        state = ParseState::InEntry;
                        current_entry = Some(Entry {
                            clinical_statement: None
                        })
                    }
                    b"act" => {
                        state = ParseState::InAct;
                        let class_code = e.try_get_attribute(b"classCode")
                            .unwrap()
                            .map(|a| String::from_utf8(a.value.to_vec()).unwrap());
                        let mood_code = e.try_get_attribute(b"moodCode")
                            .unwrap()
                            .map(|a| String::from_utf8(a.value.to_vec()).unwrap());
                        current_act = Some(EntryAct {
                            class_code,
                            mood_code,
                            act_body: Some(ActBody {
                                template_ids: Vec::new(),
                                id: None,
                                code: None,
                                status_code: None,
                                effective_time: None,
                                entry_relationships: Vec::new(),
                            })
                        });
                    }
                    b"entryRelationship" => state = ParseState::InEntryRelationship,
                    b"author" => state = ParseState::InAuthor,
                    _ => {}
                }
            }
            // works for closing tag for example </act>
            Ok(Event::End(e)) => {
                // println!("event end name {:?}", e.name());
                match e.name().as_ref() {
                    b"section" => state = ParseState::Root,
                    b"entry" => {
                        state = ParseState::InSection;
                        if let Some(entry) = current_entry.take() {
                            section.entries.push(entry);
                        }
                    }
                    b"act" =>{ 
                        state = ParseState::InEntry;
                        if let Some(entry) = &mut current_entry {
                            entry.clinical_statement = Some(ClinicalStatement::EntryAct(
                                current_act.take().unwrap()
                            ));
                        }
                    }
                    b"entryRelationship" => state = ParseState::InAct,
                    b"author" => state = ParseState::InEntryRelationship,
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
                            // println!("InSection > templateId root => {:?}", root);
                            let extension = e.try_get_attribute(b"extension")
                                .unwrap()
                                .map(|a| String::from_utf8(a.value.to_vec()).unwrap());
                            // println!("InSection > templateId extension => {:?}", extension);
                            let template_id = BaseIdentifier {
                                root,
                                extension,
                            };
                            section.template_ids.push(template_id);
                        }
                        b"code" => {
                            let code = e.try_get_attribute(b"code")
                                .unwrap()
                                .map(|a| String::from_utf8(a.value.to_vec()).unwrap());
                            let code_system = e.try_get_attribute(b"codeSystem")
                                .unwrap()
                                .map(|a| String::from_utf8(a.value.to_vec()).unwrap());
                            let display_name = e.try_get_attribute(b"displayName")
                                .unwrap()
                                .map(|a| String::from_utf8(a.value.to_vec()).unwrap());
                            let code_system_name = e.try_get_attribute(b"codeSystemName")
                                .unwrap()
                                .map(|a| String::from_utf8(a.value.to_vec()).unwrap());
                            let null_flavor = e.try_get_attribute(b"nullFlavor")
                                .unwrap()
                                .map(|a| String::from_utf8(a.value.to_vec()).unwrap());
                            println!("InSection > code: code {:?}", code);
                            section.code = Some(Code{
                                code,
                                code_system,
                                display_name,
                                code_system_name,
                                null_flavor,
                                translations: Vec::new(),
                                xsi_type: None,
                            });

                        }
                        _ => {}
                    }
                }
                if state == ParseState::InAct {
                    match e.name().as_ref() {
                        b"templateId" => {
                            let root = e.try_get_attribute(b"root")
                                .unwrap()
                                .map(|a| String::from_utf8(a.value.to_vec()).unwrap());
                            // println!("InSection > templateId root => {:?}", root);
                            let extension = e.try_get_attribute(b"extension")
                                .unwrap()
                                .map(|a| String::from_utf8(a.value.to_vec()).unwrap());
                            // println!("InSection > templateId extension => {:?}", extension);
                            let template_id = BaseIdentifier {
                                root,
                                extension,
                            };
                            // current_act.act_body.template_ids.push(template_id);
                            if let Some(act) = &mut current_act {
                                if let Some(body) = &mut act.act_body {
                                    body.template_ids.push(template_id);
                                }
                            }
                        }
                        b"id" => {
                            let root = e.try_get_attribute(b"root")
                                .unwrap()
                                .map(|a| String::from_utf8(a.value.to_vec()).unwrap());
                            // println!("InSection > templateId root => {:?}", root);
                            let extension = e.try_get_attribute(b"extension")
                                .unwrap()
                                .map(|a| String::from_utf8(a.value.to_vec()).unwrap());
                            // println!("InSection > templateId extension => {:?}", extension);
                            let id = Some(BaseIdentifier {
                                root,
                                extension,
                            });
                            // current_act.act_body.id = id;
                            if let Some(act) = &mut current_act {
                                if let Some(body) = &mut act.act_body {
                                    body.id = id;
                                }
                            }
                        }
                        b"code" => {
                            let code = e.try_get_attribute(b"code")
                                .unwrap()
                                .map(|a| String::from_utf8(a.value.to_vec()).unwrap());
                            let code_system = e.try_get_attribute(b"codeSystem")
                                .unwrap()
                                .map(|a| String::from_utf8(a.value.to_vec()).unwrap());
                            let display_name = e.try_get_attribute(b"displayName")
                                .unwrap()
                                .map(|a| String::from_utf8(a.value.to_vec()).unwrap());
                            let code_system_name = e.try_get_attribute(b"codeSystemName")
                                .unwrap()
                                .map(|a| String::from_utf8(a.value.to_vec()).unwrap());
                            let null_flavor = e.try_get_attribute(b"nullFlavor")
                                .unwrap()
                                .map(|a| String::from_utf8(a.value.to_vec()).unwrap());
                            let code = Some(Code{
                                code,
                                code_system,
                                display_name,
                                code_system_name,
                                null_flavor,
                                translations: Vec::new(),
                                xsi_type: None,
                            });
                            if let Some(act) = &mut current_act {
                                if let Some(body) = &mut act.act_body {
                                    body.code = code;
                                }
                            }
                        }
                        b"statusCode" => {
                            let code = e.try_get_attribute(b"code")
                                .unwrap()
                                .map(|a| String::from_utf8(a.value.to_vec()).unwrap());
                            if let Some(act) = &mut current_act {
                                if let Some(body) = &mut act.act_body {
                                    body.status_code = code;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            Ok(Event::Eof) => break,
            // skip other events
            _ => (),
        }
        buf.clear();
    }
    // println!("section.template_ids {:?}", section.template_ids);
    // println!("section.code {:?}", section.code);
    println!("section.entries {:?}", section.entries);
    return section;
    // todo!()
}