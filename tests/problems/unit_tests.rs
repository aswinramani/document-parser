use clinical_document_parser::parser::problem_section;
use clinical_document_parser::utils::clinical_sections::ClinicalStatement;

#[test]
fn active_problem_act_status_is_active() {
    let section = problem_section("sample-documents/problems/activeProblem.xml");
    let entry = &section.entries[0];
    let ClinicalStatement::EntryAct(act) = entry.clinical_statement.as_ref().unwrap();
    let act_body = act.act_body.as_ref().unwrap();
    assert_eq!(act_body.status_code, Some("active".to_string()));
}
