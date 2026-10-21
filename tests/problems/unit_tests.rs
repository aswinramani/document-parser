use clinical_document_parser::parser::problem_section;
use clinical_document_parser::utils::clinical_sections::{ClinicalStatement, ActBody};
use crate::fixtures::{ACTIVE_PROBLEM, RESOLVED_PROBLEM};

fn get_act_body(file: &str) -> ActBody {
    let section = problem_section(file);
    let entry = section.entries.into_iter().next().unwrap();
    let ClinicalStatement::EntryAct(act) = entry.clinical_statement.unwrap();
    act.act_body.unwrap()
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