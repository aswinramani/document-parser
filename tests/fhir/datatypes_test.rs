use clinical_document_parser::fhir::datatypes::{CodeableConcept, Coding};
use serde_json::json;

#[test]
fn coding_omits_missing_fields() {
    let coding = Coding {
        system: Some("http://terminology.hl7.org/CodeSystem/condition-clinical".to_string()),
        code: Some("active".to_string()),
        display: None,
    };
    assert_eq!(
        serde_json::to_value(&coding).unwrap(),
        json!({
            "system": "http://terminology.hl7.org/CodeSystem/condition-clinical",
            "code": "active"
        })
    );
}

#[test]
fn codeable_concept_matches_the_fixture_code_field() {
    let concept = CodeableConcept {
        coding: vec![Coding {
            system: Some("http://snomed.info/sct".to_string()),
            code: Some("385093006".to_string()),
            display: Some("Community acquired pneumonia".to_string()),
        }],
        text: Some("Community acquired pneumonia".to_string()),
    };
    assert_eq!(
        serde_json::to_value(&concept).unwrap(),
        json!({
            "coding": [{
                "system": "http://snomed.info/sct",
                "code": "385093006",
                "display": "Community acquired pneumonia"
            }],
            "text": "Community acquired pneumonia"
        })
    );
}
