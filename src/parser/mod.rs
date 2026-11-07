pub mod error;
use error::ParseError;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use quick_xml::Reader;
use quick_xml::events::Event;
use quick_xml::events::BytesStart;
use quick_xml::events::BytesEnd;
use quick_xml::XmlVersion;
use quick_xml::events::attributes::Attribute;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ParseStats {
    pub entries: usize,
    pub observations: usize,
    pub bytes_read: u64,
}


const TEXT_ELEMENTS: [&str; 10] = [
    "title", "streetAddressLine", "city", "state", "postalCode",
    "country", "given", "family", "prefix", "suffix",
];

const DOCUMENT_WRAPPERS: [&str; 3] = ["ClinicalDocument", "component", "structuredBody"];


fn get_attr(e: &BytesStart, attr: &str) -> Option<String> {
    e.try_get_attribute(attr)
        .ok()
        .flatten()
        .and_then(|a| a.normalized_value(XmlVersion::Implicit1_0).ok())
        .map(|v| v.into_owned())
}

fn attr_value(attr: &Attribute) -> Option<String> {
    attr.normalized_value(XmlVersion::Implicit1_0)
        .ok()
        .map(|v| v.into_owned())
}


fn get_time_point(e: &BytesStart) -> TimePoint {
    let mut value = None;
    let mut null_flavor = None;
    for attr in e.attributes().with_checks(false).flatten() {
        let slot = match attr.key.as_ref() {
            "value" => &mut value,
            "nullFlavor" => &mut null_flavor,
            _ => continue,
        };
        if slot.is_none() {
            *slot = attr_value(&attr);
        }
    }
    match (value, null_flavor) {
        (Some(v), _) => TimePoint::Value(v),
        (None, Some(nf)) => TimePoint::NullFlavor(nf),
        (None, None) => TimePoint::Unspecified,
    }
}

fn get_code(e: &BytesStart) -> Code {
    let mut code = Code {
        code: None,
        code_system: None,
        display_name: None,
        code_system_name: None,
        null_flavor: None,
        translations: Vec::new(),
        xsi_type: None,
    };
    for attr in e.attributes().with_checks(false).flatten() {
        let slot = match attr.key.as_ref() {
            "code" => &mut code.code,
            "codeSystem" => &mut code.code_system,
            "displayName" => &mut code.display_name,
            "codeSystemName" => &mut code.code_system_name,
            "nullFlavor" => &mut code.null_flavor,
            "xsi:type" => &mut code.xsi_type,
            _ => continue,
        };
        if slot.is_none() {
            *slot = attr
                .normalized_value(XmlVersion::Implicit1_0)
                .ok()
                .map(|v| v.into_owned());
        }
    }
    code
}

fn get_effective_time(e: &BytesStart) -> EffectiveTime {
    let mut effective_time = EffectiveTime {
        null_flavor: None,
        value: None,
        low: None,
        high: None,
    };
    for attr in e.attributes().with_checks(false).flatten() {
        let slot = match attr.key.as_ref() {
            "nullFlavor" => &mut effective_time.null_flavor,
            "value" => &mut effective_time.value,
            _ => continue,
        };
        if slot.is_none() {
            *slot = attr_value(&attr);
        }
    }
    effective_time
}

fn get_translation(e: &BytesStart) -> Translation {
    let mut translation = Translation {
        code: None,
        code_system: None,
        display_name: None,
        code_system_name: None,
        null_flavor: None,
        xsi_type: None,
    };
    for attr in e.attributes().with_checks(false).flatten() {
        let slot = match attr.key.as_ref() {
            "code" => &mut translation.code,
            "codeSystem" => &mut translation.code_system,
            "displayName" => &mut translation.display_name,
            "codeSystemName" => &mut translation.code_system_name,
            "nullFlavor" => &mut translation.null_flavor,
            "xsi:type" => &mut translation.xsi_type,
            _ => continue,
        };
        if slot.is_none() {
            *slot = attr_value(&attr);
        }
    }
    translation
}

pub fn problem_section(file_path_str: &str) -> Result<Section, ParseError> {
    let file = File::open(Path::new(file_path_str)).map_err(|source| ParseError::Io {
        path: file_path_str.to_string(),
        source,
    })?;
    problem_section_from_reader(BufReader::new(file)).map_err(|error| match error {
        ParseError::NoProblemsSection { .. } => ParseError::NoProblemsSection {
            path: Some(file_path_str.to_string()),
        },
        other => other,
    })
}

pub fn problem_section_from_reader<R: BufRead>(reader: R) -> Result<Section, ParseError> {
    problem_section_with_stats_from_reader(reader).map(|(section, _)| section)
}

pub fn problem_section_with_stats_from_reader<R: BufRead>(reader: R) -> Result<(Section, ParseStats), ParseError> {
    let _span = tracing::debug_span!("problem_section").entered();
    let mut xml = Reader::from_reader(reader);
    xml.config_mut().expand_empty_elements = true;
    let mut buf = Vec::new();
    let mut text_buf = Vec::new();
    let mut skip_buf = Vec::new();
    let mut state = ParseState::Root;
    let mut template_ids_before_section = 0;
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
    let mut observations = 0;
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
                    name if state == ParseState::Root
                        && name != "section"
                        && !DOCUMENT_WRAPPERS.contains(&name) =>
                    {
                        let end = e.to_end().into_owned();
                        if let Err(source) = xml.read_to_end_into(end.name(), &mut skip_buf) {
                            return Err(ParseError::Xml {
                                position: xml.error_position(),
                                source,
                            });
                        }
                    }
                    name if state == ParseState::Root && DOCUMENT_WRAPPERS.contains(&name) => {}
                    "section" => {
                        state = ParseState::InSection;
                        template_ids_before_section = section.template_ids.len();
                    }
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
                            observations += 1;
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
                        let code = get_code(&e);
                        match state {
                            ParseState::InSection => {
                                if code.code.as_deref() == Some("11450-4") {
                                    section.code = Some(code);
                                } else {
                                    // not the Problems section: drop what we collected and skip it whole
                                    section.template_ids.truncate(template_ids_before_section);
                                    let end = BytesEnd::new("section");
                                    if let Err(source) = xml.read_to_end_into(end.name(), &mut skip_buf) {
                                        return Err(ParseError::Xml {
                                            position: xml.error_position(),
                                            source,
                                        });
                                    }
                                    state = ParseState::Root;
                                }
                            }
                            ParseState::InAct => {
                                if let Some(act) = act_stack.last_mut()
                                    && let Some(body) = &mut act.act_body {
                                        body.code = Some(code);
                                    }
                            }
                            ParseState::InObservation => {
                                if let Some(observation) = observation_stack.last_mut() {
                                    observation.code = Some(code);
                                }
                            }
                            ParseState::InAssignedAuthor => {
                                if let Some(observation) = observation_stack.last_mut()
                                    && let Some(observation_author) = observation.author.as_mut()
                                    && let Some(assigned) = observation_author.assigned_author.as_mut() {
                                        assigned.code = Some(code);
                                    }
                            }
                            _ => {}
                        }
                    }
                    "text" => {
                        if state == ParseState::InObservation {
                            state = ParseState::InText;
                        } else if state == ParseState::InSection {
                            let end = e.to_end().into_owned();
                            match xml.read_text_into(end.name(), &mut text_buf) {
                                Ok(body) => section.text = Some(body.into_inner().into_owned()),
                                Err(source) => {
                                    return Err(ParseError::Xml {
                                        position: xml.error_position(),
                                        source,
                                    });
                                }
                            }
                        }
                    }
                    "value" => {
                        if state == ParseState::InObservation {
                            let is_quantity = get_attr(&e, "xsi:type").as_deref() == Some("PQ");
                            let content = if is_quantity {
                                Some(ValueContent::Quantity(PhysicalQuantity {
                                    value: get_attr(&e, "value"),
                                    unit: get_attr(&e, "unit"),
                                }))
                            } else {
                                Some(ValueContent::Coded(get_code(&e)))
                            };
                            if let Some(observation) = observation_stack.last_mut() {
                                observation.value = Some(Value {
                                    content,
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
                    "templateId" => {
                        let root = get_attr(&e, "root");
                        let extension = get_attr(&e, "extension");
                        let template_id = BaseIdentifier {
                            root,
                            extension,
                        };
                        match state {
                            ParseState::InSection => section.template_ids.push(template_id),
                            ParseState::InAct => {
                                if let Some(act) = act_stack.last_mut()
                                    && let Some(body) = &mut act.act_body {
                                        body.template_ids.push(template_id);
                                    }
                            }
                            ParseState::InObservation => {
                                if let Some(observation) = observation_stack.last_mut() {
                                    observation.template_ids.push(template_id);
                                }
                            }
                            ParseState::InAuthor => {
                                if let Some(observation) = observation_stack.last_mut()
                                    && let Some(observation_author) = observation.author.as_mut() {
                                        observation_author.template_id = Some(template_id);
                                    }
                            }
                            _ => {}
                        }
                    }
                    "id" => {
                        let root = get_attr(&e, "root");
                        let extension = get_attr(&e, "extension");
                        let id = BaseIdentifier {
                            root,
                            extension,
                        };
                        match state {
                            ParseState::InAct => {
                                if let Some(act) = act_stack.last_mut()
                                    && let Some(body) = &mut act.act_body {
                                        body.id = Some(id);
                                    }
                            }
                            ParseState::InObservation => {
                                if let Some(observation) = observation_stack.last_mut() {
                                    observation.id = Some(id);
                                }
                            }
                            ParseState::InAssignedAuthor => {
                                let assigned_author = Some(AssignedAuthor {
                                    id: Some(id),
                                    code: None,
                                    address: None,
                                    telecom: None,
                                    assigned_person: None,
                                });
                                if let Some(observation) = observation_stack.last_mut()
                                    && let Some(observation_author) = observation.author.as_mut() {
                                        observation_author.assigned_author = assigned_author;
                                    }
                            }
                            _ => {}
                        }
                    }
                    "statusCode" => {
                        let code = get_attr(&e, "code");
                        match state {
                            ParseState::InAct => {
                                if let Some(act) = act_stack.last_mut()
                                    && let Some(body) = &mut act.act_body {
                                        body.status_code = code;
                                    }
                            }
                            ParseState::InObservation => {
                                if let Some(observation) = observation_stack.last_mut() {
                                    observation.status_code = code;
                                }
                            }
                            _ => {}
                        }
                    }
                    "low" => {
                        if state == ParseState::InEffectiveTime
                            && let Some(effective_time) = &mut current_effective_time
                        {
                            effective_time.low = Some(get_time_point(&e));
                        }
                    }
                    "high" => {
                        if state == ParseState::InEffectiveTime
                            && let Some(effective_time) = &mut current_effective_time
                        {
                            effective_time.high = Some(get_time_point(&e));
                        }
                    }
                    "reference" => {
                        let reference = Some(Reference {
                            value: get_attr(&e, "value"),
                        });
                        match state {
                            ParseState::InText => {
                                if let Some(observation) = observation_stack.last_mut() {
                                    observation.text = reference;
                                }
                            }
                            ParseState::InOriginalText => {
                                if let Some(observation) = observation_stack.last_mut()
                                    && let Some(observation_value) = observation.value.as_mut() {
                                        observation_value.original_text = reference;
                                    }
                            }
                            _ => {}
                        }
                    }
                    "translation" => {
                        match state {
                            ParseState::InObservation => {
                                if let Some(observation) = observation_stack.last_mut()
                                    && let Some(observation_code) = observation.code.as_mut()
                                {
                                    observation_code.translations.push(get_translation(&e));
                                }
                            }
                            ParseState::InValue => {
                                if let Some(observation) = observation_stack.last_mut()
                                    && let Some(observation_value) = observation.value.as_mut()
                                    && let Some(ValueContent::Coded(value_code)) = observation_value.content.as_mut()
                                {
                                    value_code.translations.push(get_translation(&e));
                                }
                            }
                            _ => {}
                        }
                    }
                    "time" => {
                        if state == ParseState::InAuthor
                            && let Some(observation) = observation_stack.last_mut()
                            && let Some(observation_author) = observation.author.as_mut()
                        {
                            observation_author.time = Some(get_time_point(&e));
                        }
                    }
                    "telecom" => {
                        if state == ParseState::InAssignedAuthor {
                            let telecom = Some(Telecom {
                                telecom_use: get_attr(&e, "use"),
                                value: get_attr(&e, "value"),
                            });
                            if let Some(observation) = observation_stack.last_mut()
                                && let Some(observation_author) = observation.author.as_mut()
                                && let Some(observation_assigned_author) = observation_author.assigned_author.as_mut() {
                                    observation_assigned_author.telecom = telecom;
                                }
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
                    name if !TEXT_ELEMENTS.contains(&name) => {
                        let end = e.to_end().into_owned();
                        if let Err(source) = xml.read_to_end_into(end.name(), &mut skip_buf) {
                            return Err(ParseError::Xml {
                                position: xml.error_position(),
                                source,
                            });
                        }
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
        return Err(ParseError::NoProblemsSection { path: None });
    }
    let stats = ParseStats {
        entries: section.entries.len(),
        observations,
        bytes_read: xml.buffer_position(),
    };
    tracing::debug!(
        entries = stats.entries,
        observations = stats.observations,
        bytes_read = stats.bytes_read,
        "parsed problems section"
    );
    Ok((section, stats))
}