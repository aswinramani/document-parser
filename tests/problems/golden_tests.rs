use clinical_document_parser::parser::problem_section;
use clinical_document_parser::utils::clinical_sections::*;
use clinical_document_parser::utils::common_structs::*;
use crate::fixtures::{ACTIVE_PROBLEM, RESOLVED_PROBLEM, NESTED_RELATIONSHIPS, AUTHOR_WITHOUT_TEMPLATE_ID, ACT_WITH_CHILDREN, DATE_OF_DIAGNOSIS};

fn s(text: &str) -> Option<String> {
    Some(text.to_string())
}

fn base_id(root: &str, extension: Option<&str>) -> BaseIdentifier {
    BaseIdentifier {
        root: s(root),
        extension: extension.map(|e| e.to_string()),
    }
}

fn take_narrative(section: &mut Section, expected_snippet: &str) {
    let text = section
        .text
        .take()
        .expect("section text should be captured");
    assert!(
        text.contains(expected_snippet),
        "narrative should contain {expected_snippet:?}, got: {text}"
    );
}


#[test]
fn active_problem_matches_expected_section() {
    let mut actual = problem_section(ACTIVE_PROBLEM).expect("activeProblem should parse");
    take_narrative(&mut actual, "Community Acquired Pneumonia");
    let expected = Section {
        template_ids: vec![
            base_id("2.16.840.1.113883.10.20.22.2.5.1", None),
            base_id("2.16.840.1.113883.10.20.22.2.5.1", Some("2015-08-01")),
        ],
        code: Some(Code {
            code: s("11450-4"),
            display_name: s("Problem List"),
            code_system: s("2.16.840.1.113883.6.1"),
            code_system_name: None,
            null_flavor: None,
            xsi_type: None,
            translations: vec![],
        }),
        title: s("Problem List"),
        text: None,
        entries: vec![Entry {
            clinical_statement: Some(ClinicalStatement::EntryAct(EntryAct {
                class_code: s("ACT"),
                mood_code: s("EVN"),
                act_body: Some(ActBody {
                    template_ids: vec![
                        base_id("2.16.840.1.113883.10.20.22.4.3", None),
                        base_id("2.16.840.1.113883.10.20.22.4.3", Some("2015-08-01")),
                    ],
                    id: Some(base_id("102ca2e9-884c-4523-a2b4-1b6c3469c397", None)),
                    code: Some(Code {
                        code: s("CONC"),
                        display_name: None,
                        code_system: s("2.16.840.1.113883.5.6"),
                        code_system_name: None,
                        null_flavor: None,
                        xsi_type: None,
                        translations: vec![],
                    }),
                    status_code: s("active"),
                    effective_time: Some(EffectiveTime {
                        low: Some(TimePoint::Value("20140302124536-0500".to_string())),
                        high: None,
                        null_flavor: None,
                        value: None,
                    }),
                    entry_relationships: vec![EntryRelationship {
                        type_code: s("SUBJ"),
                        inversion_ind: None,
                        observation: Some(Observation {
                            class_code: s("OBS"),
                            mood_code: s("EVN"),
                            template_ids: vec![
                                base_id("2.16.840.1.113883.10.20.22.4.4", None),
                                base_id("2.16.840.1.113883.10.20.22.4.4", Some("2015-08-01")),
                            ],
                            id: Some(base_id("1.3.6.1.4.1.22812.4.111.0.4.1.2.1", Some("10241104348"))),
                            code: Some(Code {
                                code: s("55607006"),
                                display_name: s("Problem"),
                                code_system: s("2.16.840.1.113883.6.96"),
                                code_system_name: s("SNOMED CT"),
                                null_flavor: None,
                                xsi_type: None,
                                translations: vec![Translation {
                                    xsi_type: None,
                                    code: s("75326-9"),
                                    display_name: s("Problem"),
                                    code_system: s("2.16.840.1.113883.6.1"),
                                    code_system_name: s("LOINC"),
                                    null_flavor: None,
                                }],
                            }),
                            status_code: s("completed"),
                            text: Some(Reference { value: s("#Problem1") }),
                            effective_time: Some(EffectiveTime {
                                low: Some(TimePoint::Value("20140227".to_string())),
                                high: None,
                                null_flavor: None,
                                value: None,
                            }),
                            value: Some(Value {
                                content: Some(ValueContent::Coded(Code {
                                    code: s("385093006"),
                                    display_name: s("Community acquired pneumonia"),
                                    code_system: s("2.16.840.1.113883.6.96"),
                                    code_system_name: s("SNOMED CT"),
                                    null_flavor: None,
                                    xsi_type: s("CD"),
                                    translations: vec![],
                                })),
                                original_text: None,
                            }),
                            author: Some(Author {
                                template_id: Some(base_id("2.16.840.1.113883.10.20.22.4.119", None)),
                                time: Some(EffectiveTime {
                                    low: None,
                                    high: None,
                                    null_flavor: None,
                                    value: s("20140302124536"),
                                }),
                                assigned_author: Some(AssignedAuthor {
                                    id: Some(base_id("2.16.840.1.113883.4.6", Some("66666"))),
                                    code: Some(Code {
                                        code: s("207RC0000X"),
                                        display_name: s("Allopathic & Osteopathic Physicians; Internal Medicine, Cardiovascular Disease"),
                                        code_system: s("2.16.840.1.113883.6.101"),
                                        code_system_name: s("NUCC"),
                                        null_flavor: None,
                                        xsi_type: None,
                                        translations: vec![],
                                    }),
                                    address: Some(Address {
                                        address_use: None,
                                        street_address_line: vec!["6666 StreetName St.".to_string()],
                                        city: s("Silver Spring"),
                                        state: s("MD"),
                                        postal_code: s("20901"),
                                        country: s("US"),
                                    }),
                                    telecom: Some(Telecom {
                                        telecom_use: s("WP"),
                                        value: s("tel:+1(301)666-6666"),
                                    }),
                                    assigned_person: Some(PersonName {
                                        prefix: None,
                                        suffix: s("MD"),
                                        family: s("Sixer"),
                                        name_use: None,
                                        given: vec!["Heartly".to_string()],
                                    }),
                                }),
                            }),
                            entry_relationships: vec![],
                        }),
                        act: None,
                    }],
                }),
            })),
        }],
    };

    assert_eq!(actual, expected);
}

#[test]
fn date_of_diagnosis_matches_expected_section() {
    let actual = problem_section(DATE_OF_DIAGNOSIS).expect("dateOfDiagnosis should parse");

    let expected = Section {
        template_ids: vec![base_id("2.16.840.1.113883.10.20.22.2.5.1", None)],
        code: Some(Code {
            code: s("11450-4"),
            display_name: s("Problem List"),
            code_system: s("2.16.840.1.113883.6.1"),
            code_system_name: None,
            null_flavor: None,
            xsi_type: None,
            translations: vec![],
        }),
        title: s("Problem List"),
        text: None,
        entries: vec![Entry {
            clinical_statement: Some(ClinicalStatement::EntryAct(EntryAct {
                class_code: s("ACT"),
                mood_code: s("EVN"),
                act_body: Some(ActBody {
                    template_ids: vec![base_id("2.16.840.1.113883.10.20.22.4.3", None)],
                    id: Some(base_id("outer-concern-act", None)),
                    code: Some(Code {
                        code: s("CONC"),
                        display_name: None,
                        code_system: s("2.16.840.1.113883.5.6"),
                        code_system_name: None,
                        null_flavor: None,
                        xsi_type: None,
                        translations: vec![],
                    }),
                    status_code: s("active"),
                    effective_time: Some(EffectiveTime {
                        low: Some(TimePoint::Value("20140302".to_string())),
                        high: None,
                        null_flavor: None,
                        value: None,
                    }),
                    entry_relationships: vec![EntryRelationship {
                        type_code: s("SUBJ"),
                        inversion_ind: None,
                        act: None,
                        observation: Some(Observation {
                            class_code: s("OBS"),
                            mood_code: s("EVN"),
                            template_ids: vec![base_id("2.16.840.1.113883.10.20.22.4.4", None)],
                            id: Some(base_id("problem-observation", None)),
                            code: None,
                            status_code: s("completed"),
                            text: None,
                            effective_time: Some(EffectiveTime {
                                low: Some(TimePoint::Value("20140227".to_string())),
                                high: None,
                                null_flavor: None,
                                value: None,
                            }),
                            value: Some(Value {
                                content: Some(ValueContent::Coded(Code {
                                    code: s("385093006"),
                                    display_name: s("Community acquired pneumonia"),
                                    code_system: s("2.16.840.1.113883.6.96"),
                                    code_system_name: None,
                                    null_flavor: None,
                                    xsi_type: s("CD"),
                                    translations: vec![],
                                })),
                                original_text: None,
                            }),
                            author: None,
                            entry_relationships: vec![EntryRelationship {
                                type_code: s("COMP"),
                                inversion_ind: None,
                                observation: None,
                                act: Some(EntryAct {
                                    class_code: s("ACT"),
                                    mood_code: s("EVN"),
                                    act_body: Some(ActBody {
                                        template_ids: vec![base_id("2.16.840.1.113883.10.20.22.4.502", None)],
                                        id: Some(base_id("date-of-diagnosis-act", None)),
                                        code: Some(Code {
                                            code: s("77975-1"),
                                            display_name: s("Date of diagnosis"),
                                            code_system: s("2.16.840.1.113883.6.1"),
                                            code_system_name: None,
                                            null_flavor: None,
                                            xsi_type: None,
                                            translations: vec![],
                                        }),
                                        status_code: None,
                                        effective_time: Some(EffectiveTime {
                                            low: Some(TimePoint::Value("20140301".to_string())),
                                            high: None,
                                            null_flavor: None,
                                            value: None,
                                        }),
                                        entry_relationships: vec![],
                                    }),
                                }),
                            }],
                        }),
                    }],
                }),
            })),
        }],
    };

    assert_eq!(actual, expected);
}

#[test]
fn resolved_problem_matches_expected_section() {
    let mut actual = problem_section(RESOLVED_PROBLEM).expect("resolvedProblem should parse");
    take_narrative(&mut actual, "Acute pharyngitis");
    let expected = Section {
        template_ids: vec![
            base_id("2.16.840.1.113883.10.20.22.2.5.1", None),
            base_id("2.16.840.1.113883.10.20.22.2.5.1", Some("2015-08-01")),
        ],
        code: Some(Code {
            code: s("11450-4"),
            display_name: s("Problem List"),
            code_system: s("2.16.840.1.113883.6.1"),
            code_system_name: None,
            null_flavor: None,
            xsi_type: None,
            translations: vec![],
        }),
        title: s("Problem List"),
        text: None,
        entries: vec![Entry {
            clinical_statement: Some(ClinicalStatement::EntryAct(EntryAct {
                class_code: s("ACT"),
                mood_code: s("EVN"),
                act_body: Some(ActBody {
                    template_ids: vec![
                        base_id("2.16.840.1.113883.10.20.22.4.3", None),
                        base_id("2.16.840.1.113883.10.20.22.4.3", Some("2015-08-01")),
                    ],
                    id: Some(base_id("e5fbc288-659f-4aeb-a5e1-eb7cc8fcdfaf", None)),
                    code: Some(Code {
                        code: s("CONC"),
                        display_name: None,
                        code_system: s("2.16.840.1.113883.5.6"),
                        code_system_name: None,
                        null_flavor: None,
                        xsi_type: None,
                        translations: vec![],
                    }),
                    status_code: s("completed"),
                    effective_time: Some(EffectiveTime {
                        low: Some(TimePoint::Value("20140403124536-0500".to_string())),
                        high: Some(TimePoint::Value("20140407102516-0500".to_string())),
                        null_flavor: None,
                        value: None,
                    }),
                    entry_relationships: vec![EntryRelationship {
                        type_code: s("SUBJ"),
                        inversion_ind: None,
                        act: None,
                        observation: Some(Observation {
                            class_code: s("OBS"),
                            mood_code: s("EVN"),
                            template_ids: vec![
                                base_id("2.16.840.1.113883.10.20.22.4.4", None),
                                base_id("2.16.840.1.113883.10.20.22.4.4", Some("2015-08-01")),
                            ],
                            id: Some(base_id("1.3.6.1.4.1.22812.4.111.0.4.1.2.1", Some("10241108"))),
                            code: Some(Code {
                                code: s("55607006"),
                                display_name: s("Problem"),
                                code_system: s("2.16.840.1.113883.6.96"),
                                code_system_name: s("SNOMED CT"),
                                null_flavor: None,
                                xsi_type: None,
                                translations: vec![Translation {
                                    xsi_type: None,
                                    code: s("75326-9"),
                                    display_name: s("Problem"),
                                    code_system: s("2.16.840.1.113883.6.1"),
                                    code_system_name: s("LOINC"),
                                    null_flavor: None,
                                }],
                            }),
                            status_code: s("completed"),
                            text: Some(Reference { value: s("#Problem1") }),
                            effective_time: Some(EffectiveTime {
                                low: Some(TimePoint::Value("20140402".to_string())),
                                high: Some(TimePoint::Value("20140405235959-0500".to_string())),
                                null_flavor: None,
                                value: None,
                            }),
                            value: Some(Value {
                                content: Some(ValueContent::Coded(Code {
                                    code: s("363746003"),
                                    display_name: s("Acute pharyngitis"),
                                    code_system: s("2.16.840.1.113883.6.96"),
                                    code_system_name: s("SNOMED CT"),
                                    null_flavor: None,
                                    xsi_type: s("CD"),
                                    translations: vec![
                                        Translation {
                                            xsi_type: s("CD"),
                                            code: s("462"),
                                            display_name: s("Acute pharyngitis"),
                                            code_system: s("2.16.840.1.113883.6.103"),
                                            code_system_name: s("ICD-9"),
                                            null_flavor: None,
                                        },
                                        Translation {
                                            xsi_type: s("CD"),
                                            code: s("J02.9"),
                                            display_name: s("Acute pharyngitis"),
                                            code_system: s("2.16.840.1.113883.6.90"),
                                            code_system_name: s("ICD-10"),
                                            null_flavor: None,
                                        },
                                    ],
                                })),
                                original_text: Some(Reference { value: s("#ProblemDescription1") }),
                            }),
                            author: Some(Author {
                                template_id: Some(base_id("2.16.840.1.113883.10.20.22.4.119", None)),
                                time: Some(EffectiveTime {
                                    low: None,
                                    high: None,
                                    null_flavor: None,
                                    value: s("20140407"),
                                }),
                                assigned_author: Some(AssignedAuthor {
                                    id: Some(base_id("2.16.840.1.113883.4.6", Some("66666"))),
                                    code: Some(Code {
                                        code: s("207RC0000X"),
                                        display_name: s("Allopathic & Osteopathic Physicians; Internal Medicine, Cardiovascular Disease"),
                                        code_system: s("2.16.840.1.113883.6.101"),
                                        code_system_name: s("NUCC"),
                                        null_flavor: None,
                                        xsi_type: None,
                                        translations: vec![],
                                    }),
                                    address: Some(Address {
                                        address_use: None,
                                        street_address_line: vec!["6666 StreetName St.".to_string()],
                                        city: s("Silver Spring"),
                                        state: s("MD"),
                                        postal_code: s("20901"),
                                        country: s("US"),
                                    }),
                                    telecom: Some(Telecom {
                                        telecom_use: s("WP"),
                                        value: s("tel:+1(301)666-6666"),
                                    }),
                                    assigned_person: Some(PersonName {
                                        prefix: None,
                                        suffix: s("MD"),
                                        family: s("Sixer"),
                                        name_use: None,
                                        given: vec!["Heartly".to_string()],
                                    }),
                                }),
                            }),
                            entry_relationships: vec![],
                        }),
                    }],
                }),
            })),
        }],
    };

    assert_eq!(actual, expected);
}

#[test]
fn nested_relationships_match_expected_section() {
    let mut actual = problem_section(NESTED_RELATIONSHIPS).expect("nestedRelationships should parse");
    take_narrative(&mut actual, "Community Acquired Pneumonia");
    let age_at_onset = EntryRelationship {
        type_code: s("SUBJ"),
        inversion_ind: s("true"),
        act: None,
        observation: Some(Observation {
            class_code: s("OBS"),
            mood_code: s("EVN"),
            template_ids: vec![base_id("2.16.840.1.113883.10.20.22.4.31", None)],
            id: None,
            code: Some(Code {
                code: s("445518008"),
                display_name: s("Age At Onset"),
                code_system: s("2.16.840.1.113883.6.96"),
                code_system_name: None,
                null_flavor: None,
                xsi_type: None,
                translations: vec![],
            }),
            status_code: s("completed"),
            text: None,
            effective_time: None,
            value: Some(Value {
                content: Some(ValueContent::Quantity(PhysicalQuantity {
                    value: s("57"),
                    unit: s("a"),
                })),
                original_text: None,
            }),
            author: None,
            entry_relationships: vec![],
        }),
    };

    let problem_status = EntryRelationship {
        type_code: s("REFR"),
        inversion_ind: None,
        act: None,
        observation: Some(Observation {
            class_code: s("OBS"),
            mood_code: s("EVN"),
            template_ids: vec![base_id("2.16.840.1.113883.10.20.22.4.6", Some("2019-06-20"))],
            id: None,
            code: Some(Code {
                code: s("33999-4"),
                display_name: s("Diagnosis status"),
                code_system: s("2.16.840.1.113883.6.1"),
                code_system_name: None,
                null_flavor: None,
                xsi_type: None,
                translations: vec![],
            }),
            status_code: s("completed"),
            text: None,
            effective_time: None,
            value: Some(Value {
                content: Some(ValueContent::Coded(Code {
                    code: s("55561003"),
                    display_name: s("Active"),
                    code_system: s("2.16.840.1.113883.6.96"),
                    code_system_name: s("SNOMED CT"),
                    null_flavor: None,
                    xsi_type: s("CD"),
                    translations: vec![],
                })),
                original_text: None,
            }),
            author: None,
            entry_relationships: vec![],
        }),
    };

    let expected = Section {
        template_ids: vec![
            base_id("2.16.840.1.113883.10.20.22.2.5.1", None),
            base_id("2.16.840.1.113883.10.20.22.2.5.1", Some("2015-08-01")),
        ],
        code: Some(Code {
            code: s("11450-4"),
            display_name: s("Problem List"),
            code_system: s("2.16.840.1.113883.6.1"),
            code_system_name: None,
            null_flavor: None,
            xsi_type: None,
            translations: vec![],
        }),
        title: s("Problem List"),
        text: None,
        entries: vec![Entry {
            clinical_statement: Some(ClinicalStatement::EntryAct(EntryAct {
                class_code: s("ACT"),
                mood_code: s("EVN"),
                act_body: Some(ActBody {
                    template_ids: vec![
                        base_id("2.16.840.1.113883.10.20.22.4.3", None),
                        base_id("2.16.840.1.113883.10.20.22.4.3", Some("2015-08-01")),
                    ],
                    id: Some(base_id("102ca2e9-884c-4523-a2b4-1b6c3469c397", None)),
                    code: Some(Code {
                        code: s("CONC"),
                        display_name: None,
                        code_system: s("2.16.840.1.113883.5.6"),
                        code_system_name: None,
                        null_flavor: None,
                        xsi_type: None,
                        translations: vec![],
                    }),
                    status_code: s("active"),
                    effective_time: Some(EffectiveTime {
                        low: Some(TimePoint::Value("20140302124536-0500".to_string())),
                        high: None,
                        null_flavor: None,
                        value: None,
                    }),
                    entry_relationships: vec![EntryRelationship {
                        type_code: s("SUBJ"),
                        inversion_ind: None,
                        act: None,
                        observation: Some(Observation {
                            class_code: s("OBS"),
                            mood_code: s("EVN"),
                            template_ids: vec![
                                base_id("2.16.840.1.113883.10.20.22.4.4", None),
                                base_id("2.16.840.1.113883.10.20.22.4.4", Some("2015-08-01")),
                            ],
                            id: Some(base_id("1.3.6.1.4.1.22812.4.111.0.4.1.2.1", Some("10241104348"))),
                            code: Some(Code {
                                code: s("55607006"),
                                display_name: s("Problem"),
                                code_system: s("2.16.840.1.113883.6.96"),
                                code_system_name: s("SNOMED CT"),
                                null_flavor: None,
                                xsi_type: None,
                                translations: vec![Translation {
                                    xsi_type: None,
                                    code: s("75326-9"),
                                    display_name: s("Problem"),
                                    code_system: s("2.16.840.1.113883.6.1"),
                                    code_system_name: s("LOINC"),
                                    null_flavor: None,
                                }],
                            }),
                            status_code: s("completed"),
                            text: Some(Reference { value: s("#Problem1") }),
                            effective_time: Some(EffectiveTime {
                                low: Some(TimePoint::Value("20140227".to_string())),
                                high: None,
                                null_flavor: None,
                                value: None,
                            }),
                            value: Some(Value {
                                content: Some(ValueContent::Coded(Code {
                                    code: s("385093006"),
                                    display_name: s("Community acquired pneumonia"),
                                    code_system: s("2.16.840.1.113883.6.96"),
                                    code_system_name: s("SNOMED CT"),
                                    null_flavor: None,
                                    xsi_type: s("CD"),
                                    translations: vec![],
                                })),
                                original_text: None,
                            }),
                            author: Some(Author {
                                template_id: Some(base_id("2.16.840.1.113883.10.20.22.4.119", None)),
                                time: Some(EffectiveTime {
                                    low: None,
                                    high: None,
                                    null_flavor: None,
                                    value: s("20140302124536"),
                                }),
                                assigned_author: Some(AssignedAuthor {
                                    id: Some(base_id("2.16.840.1.113883.4.6", Some("66666"))),
                                    code: Some(Code {
                                        code: s("207RC0000X"),
                                        display_name: s("Allopathic & Osteopathic Physicians; Internal Medicine, Cardiovascular Disease"),
                                        code_system: s("2.16.840.1.113883.6.101"),
                                        code_system_name: s("NUCC"),
                                        null_flavor: None,
                                        xsi_type: None,
                                        translations: vec![],
                                    }),
                                    address: Some(Address {
                                        address_use: None,
                                        street_address_line: vec!["6666 StreetName St.".to_string()],
                                        city: s("Silver Spring"),
                                        state: s("MD"),
                                        postal_code: s("20901"),
                                        country: s("US"),
                                    }),
                                    telecom: Some(Telecom {
                                        telecom_use: s("WP"),
                                        value: s("tel:+1(301)666-6666"),
                                    }),
                                    assigned_person: Some(PersonName {
                                        prefix: None,
                                        suffix: s("MD"),
                                        family: s("Sixer"),
                                        name_use: None,
                                        given: vec!["Heartly".to_string()],
                                    }),
                                }),
                            }),
                            entry_relationships: vec![age_at_onset, problem_status],
                        }),
                    }],
                }),
            })),
        }],
    };

    assert_eq!(actual, expected);
}

#[test]
fn act_with_children_matches_expected_section() {
    let actual = problem_section(ACT_WITH_CHILDREN).expect("actWithChildren should parse");

    let expected = Section {
        template_ids: vec![base_id("2.16.840.1.113883.10.20.22.2.5.1", None)],
        code: Some(Code {
            code: s("11450-4"),
            display_name: s("Problem List"),
            code_system: s("2.16.840.1.113883.6.1"),
            code_system_name: None,
            null_flavor: None,
            xsi_type: None,
            translations: vec![],
        }),
        title: s("Problem List"),
        text: None,
        entries: vec![Entry {
            clinical_statement: Some(ClinicalStatement::EntryAct(EntryAct {
                class_code: s("ACT"),
                mood_code: s("EVN"),
                act_body: Some(ActBody {
                    template_ids: vec![],
                    id: Some(base_id("outer-concern-act", None)),
                    code: None,
                    status_code: s("active"),
                    effective_time: None,
                    entry_relationships: vec![EntryRelationship {
                        type_code: s("SUBJ"),
                        inversion_ind: None,
                        act: None,
                        observation: Some(Observation {
                            class_code: s("OBS"),
                            mood_code: s("EVN"),
                            template_ids: vec![],
                            id: Some(base_id("problem-observation", None)),
                            code: None,
                            status_code: s("completed"),
                            text: None,
                            effective_time: None,
                            value: None,
                            author: None,
                            entry_relationships: vec![EntryRelationship {
                                type_code: s("COMP"),
                                inversion_ind: None,
                                observation: None,
                                act: Some(EntryAct {
                                    class_code: s("ACT"),
                                    mood_code: s("EVN"),
                                    act_body: Some(ActBody {
                                        template_ids: vec![],
                                        id: Some(base_id("inner-act", None)),
                                        code: None,
                                        status_code: None,
                                        effective_time: None,
                                        entry_relationships: vec![EntryRelationship {
                                            type_code: s("COMP"),
                                            inversion_ind: None,
                                            act: None,
                                            observation: Some(Observation {
                                                class_code: s("OBS"),
                                                mood_code: s("EVN"),
                                                template_ids: vec![],
                                                id: Some(base_id("inner-observation", None)),
                                                code: None,
                                                status_code: s("completed"),
                                                text: None,
                                                effective_time: None,
                                                value: None,
                                                author: None,
                                                entry_relationships: vec![],
                                            }),
                                        }],
                                    }),
                                }),
                            }],
                        }),
                    }],
                }),
            })),
        }],
    };

    assert_eq!(actual, expected);
}

#[test]
fn author_without_template_id_matches_expected_section() {
    let actual = problem_section(AUTHOR_WITHOUT_TEMPLATE_ID).expect("authorWithoutTemplateId should parse");

    let expected = Section {
        template_ids: vec![base_id("2.16.840.1.113883.10.20.22.2.5.1", None)],
        code: Some(Code {
            code: s("11450-4"),
            display_name: s("Problem List"),
            code_system: s("2.16.840.1.113883.6.1"),
            code_system_name: None,
            null_flavor: None,
            xsi_type: None,
            translations: vec![],
        }),
        title: s("Problem List"),
        text: None,
        entries: vec![Entry {
            clinical_statement: Some(ClinicalStatement::EntryAct(EntryAct {
                class_code: s("ACT"),
                mood_code: s("EVN"),
                act_body: Some(ActBody {
                    template_ids: vec![],
                    id: None,
                    code: None,
                    status_code: s("active"),
                    effective_time: None,
                    entry_relationships: vec![EntryRelationship {
                        type_code: s("SUBJ"),
                        inversion_ind: None,
                        act: None,
                        observation: Some(Observation {
                            class_code: s("OBS"),
                            mood_code: s("EVN"),
                            template_ids: vec![],
                            id: None,
                            code: None,
                            status_code: s("completed"),
                            text: None,
                            effective_time: None,
                            value: None,
                            author: Some(Author {
                                template_id: None,
                                time: Some(EffectiveTime {
                                    low: None,
                                    high: None,
                                    null_flavor: None,
                                    value: s("20140302124536"),
                                }),
                                assigned_author: Some(AssignedAuthor {
                                    id: Some(base_id("2.16.840.1.113883.4.6", Some("66666"))),
                                    code: None,
                                    address: None,
                                    telecom: None,
                                    assigned_person: Some(PersonName {
                                        prefix: None,
                                        suffix: None,
                                        family: s("Sixer"),
                                        name_use: None,
                                        given: vec!["Heartly".to_string()],
                                    }),
                                }),
                            }),
                            entry_relationships: vec![],
                        }),
                    }],
                }),
            })),
        }],
    };

    assert_eq!(actual, expected);
}
