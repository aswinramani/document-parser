use clinical_document_parser::parser::problem_section;
use clinical_document_parser::utils::clinical_sections::{ClinicalStatement, ActBody, Observation};
use clinical_document_parser::utils::common_structs::{ValueContent};
use crate::fixtures::{ACTIVE_PROBLEM, RESOLVED_PROBLEM, NESTED_RELATIONSHIPS};

fn get_act_body(file: &str) -> ActBody {
    let section = problem_section(file).unwrap();
    let entry = section.entries.into_iter().next().unwrap();
    let ClinicalStatement::EntryAct(act) = entry.clinical_statement.unwrap();
    act.act_body.unwrap()
}

fn get_problem_observation(file: &str) -> Observation {
    get_act_body(file)
        .entry_relationships
        .into_iter()
        .next()
        .unwrap()
        .observation
        .unwrap()
}

#[test]
fn active_problem_act_status_is_active() {
    let act_body = get_act_body(ACTIVE_PROBLEM);
    assert_eq!(act_body.status_code, Some("active".to_string()));
}

#[test]
fn resolved_problem_act_effective_time_has_low_and_high() {
    let act_body = get_act_body(RESOLVED_PROBLEM);
    let effective_time = act_body.effective_time.as_ref().unwrap();
    assert_eq!(effective_time.low, Some("20140403124536-0500".to_string()));
    assert_eq!(effective_time.high, Some("20140407102516-0500".to_string()));
}

#[test]
fn nested_problem_observation_has_age_and_status_relationships() {
    let observation = get_problem_observation(NESTED_RELATIONSHIPS);
    assert_eq!(observation.entry_relationships.len(), 2);
    assert_eq!(observation.entry_relationships[0].type_code, Some("SUBJ".to_string()));
    assert_eq!(observation.entry_relationships[1].type_code, Some("REFR".to_string()));
}

#[test]
fn age_observation_value_is_a_physical_quantity() {
    let observation = get_problem_observation(NESTED_RELATIONSHIPS);
    let age = observation.entry_relationships[0].observation.as_ref().unwrap();
    let value = age.value.as_ref().unwrap();
    let ValueContent::Quantity(quantity) = value.content.as_ref().unwrap() else {
        panic!("Incorrect physical quantiy => {:?}", value.content);
    };
    assert_eq!(quantity.value, Some("57".to_string()));
    assert_eq!(quantity.unit, Some("a".to_string()));
}
