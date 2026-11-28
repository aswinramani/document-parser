use clinical_document_parser::fhir::condition::Condition;
use clinical_document_parser::fhir::datatypes::{CodeableConcept, Coding};
use serde_json::Value;
use std::fs;

#[test]
fn condition_matches_the_active_problem_fixture() {
    let condition = Condition {
        resource_type: "Condition",
        clinical_status: Some(CodeableConcept {
            coding: vec![Coding {
                system: Some("http://terminology.hl7.org/CodeSystem/condition-clinical".to_string()),
                code: Some("active".to_string()),
                display: None,
            }],
            text: None,
        }),
        code: Some(CodeableConcept {
            coding: vec![Coding {
                system: Some("http://snomed.info/sct".to_string()),
                code: Some("385093006".to_string()),
                display: Some("Community acquired pneumonia".to_string()),
            }],
            text: Some("Community acquired pneumonia".to_string()),
        }),
        onset_date_time: Some("2014-02-27".to_string()),
    };

    let raw = fs::read_to_string("sample-documents/fhir/activeProblem.conditions.json").unwrap();
    let expected: Value = serde_json::from_str(&raw).unwrap();

    assert_eq!(serde_json::to_value(&condition).unwrap(), expected[0]);
}
