use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use quick_xml::Reader;
use quick_xml::events::Event;
use quick_xml::events::BytesStart;
use crate::utils::clinical_sections::{Section, Entry,ClinicalStatement, EntryAct, ActBody, EntryRelationship, Observation};
use crate::utils::common_structs::{BaseIdentifier, Code, EffectiveTime};

#[derive(Debug, PartialEq)]
enum ParseState {
    Root,
    InSection,
    InEntry,
    InAct,
    InEffectiveTime,
    InEntryRelationship,
    InObservation,
    InAuthor,
}

fn get_attr(e: &BytesStart, attr: &[u8]) -> Option<String> {
    return e.try_get_attribute(attr)
        .ok()
        .flatten()
        .map(|a| String::from_utf8_lossy(&a.value).into_owned());
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
    let mut current_effective_time: Option<EffectiveTime> = None;
    let mut current_entry_relationship: Option<EntryRelationship> = None;
    let mut current_observation: Option<Observation> = None;
    let mut observation_depth: u8 = 0;

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
                        let class_code = get_attr(&e, b"classCode");
                        let mood_code = get_attr(&e, b"moodCode");
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
                    b"effectiveTime" => {
                        state = ParseState::InEffectiveTime;
                        let null_flavor = get_attr(&e, b"nullFlavor");
                        let value = get_attr(&e, b"value");
                        current_effective_time = Some(EffectiveTime {
                            null_flavor,
                            value,
                            low: None,
                            high: None,
                        });
                    }
                    b"entryRelationship" => {
                        if state == ParseState::InAct {
                            state = ParseState::InEntryRelationship;
                            let type_code = get_attr(&e, b"typeCode");
                            current_entry_relationship = Some(EntryRelationship {
                                type_code,
                                observation: None,
                            });
                        }
                    }
                    b"observation" => {
                        if state == ParseState::InEntryRelationship {
                            observation_depth = 1;
                            state = ParseState::InObservation;
                            let class_code = get_attr(&e, b"classCode");
                            let mood_code = get_attr(&e, b"moodCode");
                            current_observation = Some(Observation {
                                class_code,
                                mood_code,
                                template_ids: Vec::new(),
                                id: None,
                                code: None,
                                status_code: None,
                                text: None,
                                effective_time: None,
                                value: None,
                                author: None,
                            })
                        } else if state == ParseState::InObservation {
                            observation_depth += 1;
                        }
                    }
                    b"author" => state = ParseState::InAuthor,
                    _ => {}
                }
            }
            // works for closing tag for example </act>
            Ok(Event::End(e)) => {
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
                    b"effectiveTime" => {
                        state = ParseState::InAct;
                        if let Some(act) = &mut current_act {
                            if let Some(body) = &mut act.act_body {
                                body.effective_time = current_effective_time.take();
                            }
                        }
                    },
                    b"entryRelationship" => {
                        if let Some(act) = &mut current_act {
                            if let Some(body) = &mut act.act_body {
                                if let Some(some_entry_relationship) = current_entry_relationship.take() {
                                    body.entry_relationships.push(some_entry_relationship);
                                }
                            }
                        }
                        state = ParseState::InAct;
                    },
                    b"observation" => {
                        observation_depth -= 1;
                        if observation_depth == 0 {
                            if let Some(some_entry_relationship) = &mut current_entry_relationship {
                                some_entry_relationship.observation = current_observation.take();
                            }
                            state = ParseState::InEntryRelationship;
                        }
                    },
                    b"author" => state = ParseState::InObservation,
                    _ => {}
                }
            }
            // works for self-closing tag for example <templateId/>
            Ok(Event::Empty(e)) => {
                if state == ParseState::InSection {
                    match e.name().as_ref() {
                        b"templateId" => {
                            let root = get_attr(&e, b"root");
                            let extension = get_attr(&e, b"extension");
                            let template_id = BaseIdentifier {
                                root,
                                extension,
                            };
                            section.template_ids.push(template_id);
                        }
                        b"code" => {
                            let code = get_attr(&e, b"code");
                            let code_system = get_attr(&e, b"codeSystem");
                            let display_name = get_attr(&e, b"displayName");
                            let code_system_name = get_attr(&e, b"codeSystemName");
                            let null_flavor = get_attr(&e, b"nullFlavor");
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
                            let root = get_attr(&e, b"root");
                            let extension = get_attr(&e, b"extension");
                            let template_id = BaseIdentifier {
                                root,
                                extension,
                            };
                            if let Some(act) = &mut current_act {
                                if let Some(body) = &mut act.act_body {
                                    body.template_ids.push(template_id);
                                }
                            }
                        }
                        b"id" => {
                            let root = get_attr(&e, b"root");
                            let extension = get_attr(&e, b"extension");
                            let id = Some(BaseIdentifier {
                                root,
                                extension,
                            });
                            if let Some(act) = &mut current_act {
                                if let Some(body) = &mut act.act_body {
                                    body.id = id;
                                }
                            }
                        }
                        b"code" => {
                            let code = get_attr(&e, b"code");
                            let code_system = get_attr(&e, b"codeSystem");
                            let display_name = get_attr(&e, b"displayName");
                            let code_system_name = get_attr(&e, b"codeSystemName");
                            let null_flavor = get_attr(&e, b"nullFlavor");
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
                            let code = get_attr(&e, b"code");
                            if let Some(act) = &mut current_act {
                                if let Some(body) = &mut act.act_body {
                                    body.status_code = code;
                                }
                            }
                        }
                        _ => {}
                    }
                }
                if state == ParseState::InEffectiveTime {
                    match e.name().as_ref() {
                        b"low" => {
                            let low_value = get_attr(&e, b"value");
                            if let Some(effective_time) = &mut current_effective_time {
                                effective_time.low = low_value;
                            }
                        }
                        b"high" => {
                            let high_value = get_attr(&e, b"value");
                            if let Some(act) = &mut current_act {
                                if let Some(body) = &mut act.act_body {
                                    if let Some(effective_time) = &mut body.effective_time {
                                        effective_time.high = high_value;
                                    }
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
    // println!("section.code {:?}", section.code);
    println!("section.entries {:?}", section.entries);
    return section;
    // todo!()
}