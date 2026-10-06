use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use quick_xml::Reader;
use quick_xml::events::Event;
use quick_xml::events::BytesStart;
use crate::utils::clinical_sections::{Section, Entry,ClinicalStatement, EntryAct, ActBody, EntryRelationship, Observation, Value};
use crate::utils::common_structs::{BaseIdentifier, Code, EffectiveTime, Translation, Reference, Author, AssignedAuthor};

#[derive(Debug, PartialEq)]
enum ParseState {
    Root,
    InSection,
    InEntry,
    InAct,
    InEffectiveTime,
    InEntryRelationship,
    InObservation,
    InText,
    InValue,
    InOriginalText,
    InAuthor,
    InAssignedAuthor,
}

fn get_attr(e: &BytesStart, attr: &str) -> Option<String> {
    return e.try_get_attribute(attr)
        .ok()
        .flatten()
        .map(|a| a.value.into_owned());
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
    let mut entry_relationship_stack: Vec<EntryRelationship> = Vec::new();
    let mut observation_stack: Vec<Observation> = Vec::new();
    loop {
        match xml.read_event_into(&mut buf) {
            // for error handling
            Err(e) => panic!("Error occured when parsing for {:?} where byte position = {}, error position = {}", e, xml.buffer_position(), xml.error_position()),
            // works for opening tag for example <act>
            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    "section" => state = ParseState::InSection,
                    "entry" => {
                        state = ParseState::InEntry;
                        current_entry = Some(Entry {
                            clinical_statement: None
                        })
                    }
                    "act" => {
                        state = ParseState::InAct;
                        let class_code = get_attr(&e, "classCode");
                        let mood_code = get_attr(&e, "moodCode");
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
                    "effectiveTime" => {
                        state = ParseState::InEffectiveTime;
                        let null_flavor = get_attr(&e, "nullFlavor");
                        let value = get_attr(&e, "value");
                        current_effective_time = Some(EffectiveTime {
                            null_flavor,
                            value,
                            low: None,
                            high: None,
                        });
                    }
                    "entryRelationship" => {
                        if state == ParseState::InAct || state == ParseState::InObservation {
                            state = ParseState::InEntryRelationship;
                            let type_code = get_attr(&e, "typeCode");
                            entry_relationship_stack.push(EntryRelationship {
                                type_code,
                                observation: None,
                            });
                        }
                    }
                    "observation" => {
                        if state == ParseState::InEntryRelationship {
                            state = ParseState::InObservation;
                            let class_code = get_attr(&e, "classCode");
                            let mood_code = get_attr(&e, "moodCode");
                            observation_stack.push(Observation {
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
                                entry_relationships: Vec::new(),
                            });
                        }
                    }
                    "code" => {
                        if state == ParseState::InObservation {
                            let code = get_attr(&e, "code");
                            let code_system = get_attr(&e, "codeSystem");
                            let display_name = get_attr(&e, "displayName");
                            let code_system_name = get_attr(&e, "codeSystemName");
                            let null_flavor = get_attr(&e, "nullFlavor");
                            let code = Some(Code{
                                code,
                                code_system,
                                display_name,
                                code_system_name,
                                null_flavor,
                                translations: Vec::new(),
                                xsi_type: None,
                            });
                            if let Some(observation) = observation_stack.last_mut() {
                                observation.code = code;
                            }
                        }
                    }
                    "text" => {
                        if state == ParseState::InObservation {
                            state = ParseState::InText;
                        }
                    }
                    "value" => {
                        if state == ParseState::InObservation {
                            let code = get_attr(&e, "code");
                            let code_system = get_attr(&e, "codeSystem");
                            let display_name = get_attr(&e, "displayName");
                            let code_system_name = get_attr(&e, "codeSystemName");
                            let null_flavor = get_attr(&e, "nullFlavor");
                            let xsi_type = get_attr(&e, "xsi:type");
                            let code = Some(Code{
                                code,
                                code_system,
                                display_name,
                                code_system_name,
                                null_flavor,
                                translations: Vec::new(),
                                xsi_type,
                            });
                            let obs_value = Some(Value{
                                code,
                                original_text: None,
                            });
                            if let Some(observation) = observation_stack.last_mut() {
                                observation.value = obs_value;
                            }
                            state = ParseState::InValue;
                        }
                    }
                    "originalText" => {
                        if state == ParseState::InValue {
                            state = ParseState::InOriginalText;
                        }
                    }
                    "author" => state = ParseState::InAuthor,
                    "assignedAuthor" => state = ParseState::InAssignedAuthor,
                    _ => {}
                }
            }
            // works for closing tag for example </act>
            Ok(Event::End(e)) => {
                match e.name().as_ref() {
                    "section" => state = ParseState::Root,
                    "entry" => {
                        state = ParseState::InSection;
                        if let Some(entry) = current_entry.take() {
                            section.entries.push(entry);
                        }
                    }
                    "act" =>{ 
                        state = ParseState::InEntry;
                        if let Some(entry) = &mut current_entry {
                            entry.clinical_statement = Some(ClinicalStatement::EntryAct(
                                current_act.take().unwrap()
                            ));
                        }
                    }
                    "effectiveTime" => {
                        if let Some(observation) = observation_stack.last_mut() {
                            observation.effective_time = current_effective_time.take();
                            state = ParseState::InObservation;
                        } else if let Some(act) = &mut current_act {
                            if let Some(body) = &mut act.act_body {
                                body.effective_time = current_effective_time.take();
                            }
                            state = ParseState::InAct;
                        }
                    },
                    "entryRelationship" => {
                        if let Some(finished_entry_relationship) = entry_relationship_stack.pop() {
                            if let Some(waiting_observation) = observation_stack.last_mut() {
                                waiting_observation.entry_relationships.push(finished_entry_relationship);
                                state = ParseState::InObservation;
                            } else {
                                if let Some(act) = &mut current_act {
                                    if let Some(body) = &mut act.act_body {
                                        body.entry_relationships.push(finished_entry_relationship);
                                    }
                                }
                                state = ParseState::InAct;
                            }
                        }
                    },
                    "observation" => {
                        if let Some(finished_observation) = observation_stack.pop() {
                            if let Some(waiting_relationship) = entry_relationship_stack.last_mut() {
                                waiting_relationship.observation = Some(finished_observation);
                            }
                            state = ParseState::InEntryRelationship;
                        }
                    },
                    "text" => {
                        if state == ParseState::InText {
                            state = ParseState::InObservation;
                        }
                    }
                    "value" => state = ParseState::InObservation,
                    "originalText" => state = ParseState::InValue,
                    "author" => state = ParseState::InObservation,
                    "assignedAuthor" => state = ParseState::InAuthor,
                    _ => {}
                }
            }
            // works for self-closing tag for example <templateId/>
            Ok(Event::Empty(e)) => {
                if state == ParseState::InSection {
                    match e.name().as_ref() {
                        "templateId" => {
                            let root = get_attr(&e, "root");
                            let extension = get_attr(&e, "extension");
                            let template_id = BaseIdentifier {
                                root,
                                extension,
                            };
                            section.template_ids.push(template_id);
                        }
                        "code" => {
                            let code = get_attr(&e, "code");
                            let code_system = get_attr(&e, "codeSystem");
                            let display_name = get_attr(&e, "displayName");
                            let code_system_name = get_attr(&e, "codeSystemName");
                            let null_flavor = get_attr(&e, "nullFlavor");
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
                        "templateId" => {
                            let root = get_attr(&e, "root");
                            let extension = get_attr(&e, "extension");
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
                        "id" => {
                            let root = get_attr(&e, "root");
                            let extension = get_attr(&e, "extension");
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
                        "code" => {
                            let code = get_attr(&e, "code");
                            let code_system = get_attr(&e, "codeSystem");
                            let display_name = get_attr(&e, "displayName");
                            let code_system_name = get_attr(&e, "codeSystemName");
                            let null_flavor = get_attr(&e, "nullFlavor");
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
                        "statusCode" => {
                            let code = get_attr(&e, "code");
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
                        "low" => {
                            let low_value = get_attr(&e, "value");
                            if let Some(effective_time) = &mut current_effective_time {
                                effective_time.low = low_value;
                            }
                        }
                        "high" => {
                            let high_value = get_attr(&e, "value");
                            if let Some(effective_time) = &mut current_effective_time {
                                effective_time.high = high_value;
                            }
                        }
                        _ => {}
                    }
                }
                if state == ParseState::InObservation {
                    match e.name().as_ref() {
                        "templateId" => {
                            let root = get_attr(&e, "root");
                            let extension = get_attr(&e, "extension");
                            let template_id = BaseIdentifier {
                                root,
                                extension,
                            };
                            if let Some(observation) = observation_stack.last_mut() {
                                observation.template_ids.push(template_id);
                            }
                        }
                        "id" => {
                            let root = get_attr(&e, "root");
                            let extension = get_attr(&e, "extension");
                            let id = Some(BaseIdentifier {
                                root,
                                extension,
                            });
                            if let Some(observation) = observation_stack.last_mut() {
                                observation.id = id;
                            }
                        }
                        "statusCode" => {
                            let code = get_attr(&e, "code");
                            if let Some(observation) = observation_stack.last_mut() {
                                observation.status_code = code;
                            }
                        }
                        "translation" => {
                            let code = get_attr(&e, "code");
                            let code_system = get_attr(&e, "codeSystem");
                            let display_name = get_attr(&e, "displayName");
                            let code_system_name = get_attr(&e, "codeSystemName");
                            let null_flavor = get_attr(&e, "nullFlavor");
                            let xsi_type = get_attr(&e, "xsi:type");
                            let code = Translation{
                                code,
                                code_system,
                                display_name,
                                code_system_name,
                                null_flavor,
                                xsi_type,
                            };
                            if let Some(observation) = observation_stack.last_mut() {
                                if let Some(observation_code) =  observation.code.as_mut() {
                                    observation_code.translations.push(code);
                                }
                            }
                        }
                        "value" => {
                            let code = get_attr(&e, "code");
                            let code_system = get_attr(&e, "codeSystem");
                            let display_name = get_attr(&e, "displayName");
                            let code_system_name = get_attr(&e, "codeSystemName");
                            let null_flavor = get_attr(&e, "nullFlavor");
                            let xsi_type = get_attr(&e, "xsi:type");
                            let value_code = Some(Code{
                                code,
                                code_system,
                                display_name,
                                code_system_name,
                                null_flavor,
                                translations: Vec::new(),
                                xsi_type,
                            });
                            if let Some(observation) = observation_stack.last_mut() {
                                observation.value = Some(Value{
                                    code: value_code,
                                    original_text: None,
                                });
                            }
                        }

                        _ => {}
                    }
                }
                if state == ParseState::InText {
                    match e.name().as_ref() {
                        "reference" => {
                            let value = get_attr(&e, "value");
                            let reference = Some(Reference{
                                value
                            });
                            if let Some(observation) = observation_stack.last_mut() {
                                observation.text = reference;
                            }
                        }
                        _ => {}
                    }
                }
                if state == ParseState::InValue {
                    match e.name().as_ref() {
                        "translation" => {
                            let code = get_attr(&e, "code");
                            let code_system = get_attr(&e, "codeSystem");
                            let display_name = get_attr(&e, "displayName");
                            let code_system_name = get_attr(&e, "codeSystemName");
                            let null_flavor = get_attr(&e, "nullFlavor");
                            let xsi_type = get_attr(&e, "xsi:type");
                            let code = Translation{
                                code,
                                code_system,
                                display_name,
                                code_system_name,
                                null_flavor,
                                xsi_type,
                            };
                            if let Some(observation) = observation_stack.last_mut() {
                                if let Some(observation_value) =  observation.value.as_mut() {
                                    if let Some(value_code) =  observation_value.code.as_mut() {
                                        value_code.translations.push(code);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                if state == ParseState::InOriginalText {
                    match e.name().as_ref() {
                        "reference" => {
                            let value = get_attr(&e, "value");
                            let reference = Some(Reference{
                                value
                            });
                            if let Some(observation) = observation_stack.last_mut() {
                                if let Some(observation_value) =  observation.value.as_mut() {
                                    observation_value.original_text = reference;
                                }
                            }
                        }
                        _ => {}
                    }
                }
                if state == ParseState::InAuthor {
                    match e.name().as_ref() {
                        "templateId" => {
                            let root = get_attr(&e, "root");
                            let extension = get_attr(&e, "extension");
                            let template_id = Some(BaseIdentifier{
                                root,
                                extension,
                            });
                            let author = Some(Author{
                                template_id,
                                time: None,
                                assigned_author: None,
                            });
                            if let Some(observation) = observation_stack.last_mut() {
                                observation.author = author;
                            }
                        }
                        "time" => {
                            let low = get_attr(&e, "low");
                            let high = get_attr(&e, "high");
                            let null_flavor = get_attr(&e, "null_flavor");
                            let value = get_attr(&e, "value");
                            let time = Some(EffectiveTime{
                                low,
                                high,
                                null_flavor,
                                value,
                            });
                            if let Some(observation) = observation_stack.last_mut() {
                                if let Some(observation_author) =  observation.author.as_mut() {
                                    observation_author.time = time;
                                }
                            }
                        }
                        _ => {}
                    }
                }
                if state == ParseState::InAssignedAuthor {
                    match e.name().as_ref() {
                        "id" => {
                            let root = get_attr(&e, "root");
                            let extension = get_attr(&e, "extension");
                            let id = Some(BaseIdentifier{
                                root,
                                extension,
                            });
                            let assigned_author = Some(AssignedAuthor{
                                id,
                                code: None,
                                address: None,
                                telecom: None,
                                assigned_person: None,
                            });
                            if let Some(observation) = observation_stack.last_mut() {
                                if let Some(observation_author) =  observation.author.as_mut() {
                                    observation_author.assigned_author = assigned_author;
                                }
                            }
                        }
                        "code" => {
                            let code = get_attr(&e, "code");
                            let code_system = get_attr(&e, "codeSystem");
                            let display_name = get_attr(&e, "displayName");
                            let code_system_name = get_attr(&e, "codeSystemName");
                            let null_flavor = get_attr(&e, "nullFlavor");
                            let code = Some(Code{
                                code,
                                code_system,
                                display_name,
                                code_system_name,
                                null_flavor,
                                translations: Vec::new(),
                                xsi_type: None,
                            });
                            if let Some(observation) = observation_stack.last_mut() {
                                if let Some(observation_author) =  observation.author.as_mut() {
                                    if let Some(observation_assigned_author) = observation_author.assigned_author.as_mut() {
                                        observation_assigned_author.code = code;
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