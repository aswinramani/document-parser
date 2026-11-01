use clinical_document_parser::parser::problem_section;
use clinical_document_parser::utils::clinical_sections::*;
use clinical_document_parser::utils::common_structs::*;
use crate::fixtures::ACTIVE_PROBLEM;

fn s(text: &str) -> Option<String> {
    Some(text.to_string())
}

fn base_id(root: &str, extension: Option<&str>) -> BaseIdentifier {
    BaseIdentifier {
        root: s(root),
        extension: extension.map(|e| e.to_string()),
    }
}

#[test]
fn active_problem_matches_expected_section() {
    let actual = problem_section(ACTIVE_PROBLEM)
        .ok()
        .expect("activeProblem.xml should parse");

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
