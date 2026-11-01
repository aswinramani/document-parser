pub mod error;
use error::ParseError;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use quick_xml::Reader;
use quick_xml::events::Event;
use quick_xml::events::BytesStart;
use quick_xml::XmlVersion;
use crate::utils::clinical_sections::{Section, Entry,ClinicalStatement, EntryAct, ActBody, EntryRelationship, Observation, Value};
use crate::utils::common_structs::{BaseIdentifier, Code, EffectiveTime, Translation, Reference, Author, AssignedAuthor, Telecom, Address, PersonName, ValueContent, PhysicalQuantity, TimePoint};
 
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
    InAssignedPerson,
}

fn get_attr(e: &BytesStart, attr: &str) -> Option<String> {
    e.try_get_attribute(attr)
        .ok()
        .flatten()
        .and_then(|a| a.normalized_value(XmlVersion::Implicit1_0).ok())
        .map(|v| v.into_owned())
}

fn get_time_point(e: &BytesStart) -> TimePoint {
    if let Some(value) = get_attr(e, "value") {
        TimePoint::Value(value)
    } else if let Some(null_flavor) = get_attr(e, "nullFlavor") {
        TimePoint::NullFlavor(null_flavor)
    } else {
        TimePoint::Unspecified
    }
}

fn get_code(e: &BytesStart) -> Code {
    Code {
        code: get_attr(e, "code"),
        code_system: get_attr(e, "codeSystem"),
        display_name: get_attr(e, "displayName"),
        code_system_name: get_attr(e, "codeSystemName"),
        null_flavor: get_attr(e, "nullFlavor"),
        translations: Vec::new(),
        xsi_type: get_attr(e, "xsi:type"),
    }
}


fn get_effective_time(e: &BytesStart) -> EffectiveTime {
    EffectiveTime {
        null_flavor: get_attr(e, "nullFlavor"),
        value: get_attr(e, "value"),
        low: None,
        high: None,
    }
}

fn get_translation(e: &BytesStart) -> Translation {
    Translation {
        code: get_attr(e, "code"),
        code_system: get_attr(e, "codeSystem"),
        display_name: get_attr(e, "displayName"),
        code_system_name: get_attr(e, "codeSystemName"),
        null_flavor: get_attr(e, "nullFlavor"),
        xsi_type: get_attr(e, "xsi:type"),
    }
}

pub fn problem_section(file_path_str: &str) -> Result<Section, ParseError> {
    let file_path = Path::new(file_path_str);
    let file = File::open(file_path).map_err(|source| ParseError::Io {
        path: file_path_str.to_string(),
        source,
    })?;
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
    let mut act_stack: Vec<EntryAct> = Vec::new();
    let mut current_effective_time: Option<EffectiveTime> = None;
    let mut entry_relationship_stack: Vec<EntryRelationship> = Vec::new();
    let mut entry_relationship_owners: Vec<bool> = Vec::new();
    let mut observation_stack: Vec<Observation> = Vec::new();
    let mut effective_time_in_observation = false;
    let mut current_text_field: Option<String> = None;
    let mut current_address: Option<Address> = None;
    let mut current_person_name: Option<PersonName> = None;
    loop {
        match xml.read_event_into(&mut buf) {
            // for error handling
            Err(source) => return Err(ParseError::Xml {
                position: xml.error_position(),
                source,
            }),
            // works for opening tag for example <act>
            Ok(Event::Start(e)) => {
                current_text_field = Some(e.name().as_ref().to_string());
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
                        act_stack.push(EntryAct {
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
                        effective_time_in_observation = state == ParseState::InObservation;
                        state = ParseState::InEffectiveTime;
                        current_effective_time = Some(get_effective_time(&e));
                    }
                    "entryRelationship" => {
                        if state == ParseState::InAct || state == ParseState::InObservation {
                            entry_relationship_owners.push(state == ParseState::InObservation);
                            state = ParseState::InEntryRelationship;
                            let type_code = get_attr(&e, "typeCode");
                            entry_relationship_stack.push(EntryRelationship {
                                type_code,
                                inversion_ind: get_attr(&e, "inversionInd"),
                                observation: None,
                                act: None
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
                        if state == ParseState::InObservation
                            && let Some(observation) = observation_stack.last_mut()
                        {
                            observation.code = Some(get_code(&e));
                        }
                    }
                    "text" => {
                        if state == ParseState::InObservation {
                            state = ParseState::InText;
                        }
                    }
                    "value" => {
                        if state == ParseState::InObservation {
                            if let Some(observation) = observation_stack.last_mut() {
                                observation.value = Some(Value {
                                    content: Some(ValueContent::Coded(get_code(&e))),
                                    original_text: None,
                                });
                            }
                            state = ParseState::InValue;
                        }
                    }
                    "originalText" => {
                        if state == ParseState::InValue {
                            state = ParseState::InOriginalText;
                        }
                    }
                    "author" => {
                        state = ParseState::InAuthor;
                        if let Some(observation) = observation_stack.last_mut() {
                            observation.author = Some(Author {
                                template_id: None,
                                time: None,
                                assigned_author: None,
                            });
                        }
                    }
                    "assignedAuthor" => state = ParseState::InAssignedAuthor,
                    "addr" => {
                        let address_use = get_attr(&e, "use");
                        current_address = Some(Address {
                            address_use,
                            street_address_line: Vec::new(),
                            city: None,
                            state: None,
                            postal_code: None,
                            country: None,
                        });
                    }
                    "assignedPerson" => {
                        if state == ParseState::InAssignedAuthor {
                            state = ParseState::InAssignedPerson;
                        }
                    }
                    "name"
                        if state == ParseState::InAssignedPerson => {
                            let name_use = get_attr(&e, "use");
                            current_person_name = Some(PersonName {
                                prefix: None,
                                suffix: None,
                                family: None,
                                name_use,
                                given: Vec::new(),
                            })
                        }
                    _ => {}
                }
            }
            // works for closing tag for example </act>
            Ok(Event::End(e)) => {
                current_text_field = None;
                match e.name().as_ref() {
                    "section" => state = ParseState::Root,
                    "entry" => {
                        state = ParseState::InSection;
                        if let Some(entry) = current_entry.take() {
                            section.entries.push(entry);
                        }
                    }
                    "act" =>{ 
                        if let Some(act) = act_stack.pop() {
                            if let Some(relationship) = entry_relationship_stack.last_mut() {
                                relationship.act = Some(act);
                                state = ParseState::InEntryRelationship;
                            } else if let Some(entry) = current_entry.as_mut() {
                                entry.clinical_statement = Some(ClinicalStatement::EntryAct(act));
                                state = ParseState::InEntry;
                            }
                        }
                    }
                    "effectiveTime" => {
                        if effective_time_in_observation {
                            if let Some(observation) = observation_stack.last_mut() {
                                observation.effective_time = current_effective_time.take();
                                state = ParseState::InObservation;
                            }
                        } else if let Some(act) = act_stack.last_mut() {
                            if let Some(body) = &mut act.act_body {
                                body.effective_time = current_effective_time.take();
                            }
                            state = ParseState::InAct;
                        }
                    },
                    "entryRelationship" => {
                        if let Some(finished_entry_relationship) = entry_relationship_stack.pop() {
                            let owned_by_observation = entry_relationship_owners.pop().unwrap_or(false);
                            if owned_by_observation {
                                if let Some(waiting_observation) = observation_stack.last_mut() {
                                    waiting_observation.entry_relationships.push(finished_entry_relationship);
                                }
                                state = ParseState::InObservation;
                            } else {
                                if let Some(act) = act_stack.last_mut()
                                    && let Some(body) = &mut act.act_body {
                                        body.entry_relationships.push(finished_entry_relationship);
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
                    "addr" => {
                        if let Some(observation) = observation_stack.last_mut()
                            && let Some(author) = observation.author.as_mut()
                                && let Some(assigned) = author.assigned_author.as_mut() {
                                    assigned.address = current_address.take();
                                }
                    }
                    "assignedPerson" => state = ParseState::InAssignedAuthor,
                    "name" => {
                        if let Some(observation) = observation_stack.last_mut()
                            && let Some(author) = observation.author.as_mut()
                                && let Some(assigned) = author.assigned_author.as_mut() {
                                    assigned.assigned_person = current_person_name.take();
                                }
                    }
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
                            section.code = Some(get_code(&e));
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
                            if let Some(act) = act_stack.last_mut()
                                && let Some(body) = &mut act.act_body {
                                    body.template_ids.push(template_id);
                                }
                        }
                        "id" => {
                            let root = get_attr(&e, "root");
                            let extension = get_attr(&e, "extension");
                            let id = Some(BaseIdentifier {
                                root,
                                extension,
                            });
                            if let Some(act) = act_stack.last_mut()
                                && let Some(body) = &mut act.act_body {
                                    body.id = id;
                                }
                        }
                        "code" => {
                            if let Some(act) = act_stack.last_mut()
                                && let Some(body) = &mut act.act_body
                            {
                                body.code = Some(get_code(&e));
                            }
                        }
                        "statusCode" => {
                            let code = get_attr(&e, "code");
                            if let Some(act) = act_stack.last_mut()
                                && let Some(body) = &mut act.act_body {
                                    body.status_code = code;
                                }
                        }
                        "effectiveTime" => {
                            if let Some(act) = act_stack.last_mut()
                                && let Some(body) = &mut act.act_body {
                                    body.effective_time = Some(get_effective_time(&e));
                                }
                        }
                        _ => {}
                    }
                }
                if state == ParseState::InEffectiveTime {
                    match e.name().as_ref() {
                        "low" => {
                            if let Some(effective_time) = &mut current_effective_time {
                                effective_time.low = Some(get_time_point(&e));
                            }
                        }
                        "high" => {
                            if let Some(effective_time) = &mut current_effective_time {
                                effective_time.high = Some(get_time_point(&e));
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
                        "code" => {
                            if let Some(observation) = observation_stack.last_mut() {
                                observation.code = Some(get_code(&e));
                            }
                        }
                        "statusCode" => {
                            let code = get_attr(&e, "code");
                            if let Some(observation) = observation_stack.last_mut() {
                                observation.status_code = code;
                            }
                        }
                        "translation" => {
                            if let Some(observation) = observation_stack.last_mut()
                                && let Some(observation_code) = observation.code.as_mut()
                            {
                                observation_code.translations.push(get_translation(&e));
                            }
                        }
                        "value" => {
                            let xsi_type = get_attr(&e, "xsi:type");
                            let is_quantity = xsi_type.as_deref() == Some("PQ");
                            let content = if is_quantity {
                                let value = get_attr(&e, "value");
                                let unit = get_attr(&e, "unit");
                                Some(ValueContent::Quantity(PhysicalQuantity { value, unit }))
                            } else {
                                Some(ValueContent::Coded(get_code(&e)))
                            };
                            if let Some(observation) = observation_stack.last_mut() {
                                observation.value = Some(Value{
                                    content,
                                    original_text: None,
                                });
                            }
                        }
                        "effectiveTime" => {
                            if let Some(observation) = observation_stack.last_mut() {
                                observation.effective_time = Some(get_effective_time(&e));
                            }
                        }
                        _ => {}
                    }
                }
                if state == ParseState::InText
                    && e.name().as_ref() == "reference" {
                        let value = get_attr(&e, "value");
                        let reference = Some(Reference{
                            value
                        });
                        if let Some(observation) = observation_stack.last_mut() {
                            observation.text = reference;
                        }
                    }
                if state == ParseState::InValue
                    && e.name().as_ref() == "translation"
                    && let Some(observation) = observation_stack.last_mut()
                    && let Some(observation_value) = observation.value.as_mut()
                    && let Some(ValueContent::Coded(value_code)) = observation_value.content.as_mut()
                {
                    value_code.translations.push(get_translation(&e));
                }
                if state == ParseState::InOriginalText
                    && e.name().as_ref() == "reference" {
                        let value = get_attr(&e, "value");
                        let reference = Some(Reference{
                            value
                        });
                        if let Some(observation) = observation_stack.last_mut()
                            && let Some(observation_value) =  observation.value.as_mut() {
                                observation_value.original_text = reference;
                            }
                    }
                if state == ParseState::InAuthor {
                    match e.name().as_ref() {
                        "templateId" => {
                            let root = get_attr(&e, "root");
                            let extension = get_attr(&e, "extension");
                             let template_id = Some(BaseIdentifier {
                                root,
                                extension,
                            });
                            if let Some(observation) = observation_stack.last_mut()
                                && let Some(observation_author) = observation.author.as_mut() {
                                    observation_author.template_id = template_id;
                                }
                        }
                        "time" => {
                            let null_flavor = get_attr(&e, "nullFlavor");
                            let value = get_attr(&e, "value");
                            let time = Some(EffectiveTime{
                                low: None,
                                high: None,
                                null_flavor,
                                value,
                            });
                            if let Some(observation) = observation_stack.last_mut()
                                && let Some(observation_author) =  observation.author.as_mut() {
                                    observation_author.time = time;
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
                            if let Some(observation) = observation_stack.last_mut()
                                && let Some(observation_author) =  observation.author.as_mut() {
                                    observation_author.assigned_author = assigned_author;
                                }
                        }
                        "code" => {
                            if let Some(observation) = observation_stack.last_mut()
                                && let Some(observation_author) = observation.author.as_mut()
                                && let Some(observation_assigned_author) = observation_author.assigned_author.as_mut()
                            {
                                observation_assigned_author.code = Some(get_code(&e));
                            }

                        }
                        "telecom" => {
                            let value = get_attr(&e, "value");
                            let telecom_use = get_attr(&e, "use");
                            let telecom = Some(Telecom{
                                telecom_use,
                                value,
                            });
                            if let Some(observation) = observation_stack.last_mut()
                                && let Some(observation_author) =  observation.author.as_mut()
                                    && let Some(observation_assigned_author) = observation_author.assigned_author.as_mut() {
                                        observation_assigned_author.telecom = telecom;
                                    }
                        }
                        _ => {}
                    }
                }
            }
            Ok(Event::Text(e)) => {
                let text = e.xml10_content();
                if let Some(address) = current_address.as_mut() {
                    match current_text_field.as_deref() {
                        Some("streetAddressLine") => address.street_address_line.push(text.into_owned()),
                        Some("city") => address.city = Some(text.into_owned()),
                        Some("state") => address.state = Some(text.into_owned()),
                        Some("postalCode") => address.postal_code = Some(text.into_owned()),
                        Some("country") => address.country = Some(text.into_owned()),
                        _ => {}
                    }
                } else if let Some(person) = current_person_name.as_mut() {
                    match current_text_field.as_deref() {
                        Some("prefix") => person.prefix = Some(text.into_owned()),
                        Some("suffix") => person.suffix = Some(text.into_owned()),
                        Some("family") => person.family = Some(text.into_owned()),
                        Some("given") => person.given.push(text.into_owned()),
                        _ => {}
                    }
                } else if state == ParseState::InSection && current_text_field.as_deref() == Some("title") {
                    section.title = Some(text.into_owned());
                }
            }
            Ok(Event::Eof) => break,
            // skip other events
            _ => (),
        }
        buf.clear();
    }
    let has_problems_code = section
        .code
        .as_ref()
        .and_then(|c| c.code.as_deref())
        == Some("11450-4");
    if !has_problems_code {
        return Err(ParseError::NoProblemsSection {
            path: file_path_str.to_string(),
        });
    }
    Ok(section)
}