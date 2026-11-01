use clinical_document_parser::parser::problem_section;
use clinical_document_parser::utils::clinical_sections::{ClinicalStatement, ActBody, Observation};
use clinical_document_parser::utils::common_structs::{ValueContent, TimePoint};
use crate::fixtures::{ACTIVE_PROBLEM, RESOLVED_PROBLEM, NESTED_RELATIONSHIPS, EFFECTIVE_TIME_FORMS, AUTHOR_WITHOUT_TEMPLATE_ID, DATE_OF_DIAGNOSIS, ACT_WITH_CHILDREN};

fn get_act_body(file: &str, index: usize) -> ActBody {
    let section = problem_section(file).unwrap();
    let entry = section.entries.into_iter().nth(index).unwrap();
    let ClinicalStatement::EntryAct(act) = entry.clinical_statement.unwrap();
    act.act_body.unwrap()
}

fn get_problem_observation(file: &str, index: usize) -> Observation {
    get_act_body(file, index)
        .entry_relationships
        .into_iter()
        .next()
        .unwrap()
        .observation
        .unwrap()
}

#[test]
fn active_problem_act_status_is_active() {
    let act_body = get_act_body(ACTIVE_PROBLEM, 0);
    assert_eq!(act_body.status_code, Some("active".to_string()));
}

#[test]
fn resolved_problem_act_effective_time_has_low_and_high() {
    let act_body = get_act_body(RESOLVED_PROBLEM, 0);
    let effective_time = act_body.effective_time.as_ref().unwrap();
    assert_eq!(effective_time.low, Some(TimePoint::Value("20140403124536-0500".to_string())));
    assert_eq!(effective_time.high, Some(TimePoint::Value("20140407102516-0500".to_string())));
}

#[test]
fn nested_problem_observation_has_age_and_status_relationships() {
    let observation = get_problem_observation(NESTED_RELATIONSHIPS, 0);
    assert_eq!(observation.entry_relationships.len(), 2);
    assert_eq!(observation.entry_relationships[0].type_code, Some("SUBJ".to_string()));
    assert_eq!(observation.entry_relationships[1].type_code, Some("REFR".to_string()));
}

#[test]
fn age_observation_value_is_a_physical_quantity() {
    let observation = get_problem_observation(NESTED_RELATIONSHIPS, 0);
    let age = observation.entry_relationships[0].observation.as_ref().unwrap();
    let value = age.value.as_ref().unwrap();
    let ValueContent::Quantity(quantity) = value.content.as_ref().unwrap() else {
        panic!("Incorrect physical quantiy => {:?}", value.content);
    };
    assert_eq!(quantity.value, Some("57".to_string()));
    assert_eq!(quantity.unit, Some("a".to_string()));
}

#[test]
fn high_with_null_flavor_means_resolved_with_unknown_date() {
    let observation = get_problem_observation(EFFECTIVE_TIME_FORMS, 3);
    let effective_time = observation.effective_time.unwrap();
    assert_eq!(effective_time.low, Some(TimePoint::Value("20140227".to_string())));
    assert_eq!(effective_time.high, Some(TimePoint::NullFlavor("UNK".to_string())));
}

#[test]
fn act_self_closing_effective_time_keeps_null_flavor() {
    let act_body = get_act_body(EFFECTIVE_TIME_FORMS, 0);
    let effective_time = act_body.effective_time.unwrap();
    assert_eq!(effective_time.null_flavor, Some("UNK".to_string()));
    assert_eq!(effective_time.value, None);
}

#[test]
fn observation_self_closing_effective_time_keeps_value() {
    let observation = get_problem_observation(EFFECTIVE_TIME_FORMS, 1);
    let effective_time = observation.effective_time.unwrap();
    assert_eq!(effective_time.value, Some("20140227".to_string()));
    assert_eq!(effective_time.null_flavor, None);
}

#[test]
fn observation_self_closing_effective_time_keeps_null_flavor() {
    let observation = get_problem_observation(EFFECTIVE_TIME_FORMS, 2);
    let effective_time = observation.effective_time.unwrap();
    assert_eq!(effective_time.null_flavor, Some("UNK".to_string()));
    assert_eq!(effective_time.value, None);
}

#[test]
fn low_with_null_flavor_means_onset_unknown() {
    let observation = get_problem_observation(EFFECTIVE_TIME_FORMS, 4);
    let effective_time = observation.effective_time.unwrap();
    assert_eq!(effective_time.low, Some(TimePoint::NullFlavor("UNK".to_string())));
    assert_eq!(effective_time.high, None);
}

#[test]
fn bare_high_element_is_kept_as_unspecified() {
    let observation = get_problem_observation(EFFECTIVE_TIME_FORMS, 5);
    let effective_time = observation.effective_time.unwrap();
    assert_eq!(effective_time.low, Some(TimePoint::Value("20140227".to_string())));
    assert_eq!(effective_time.high, Some(TimePoint::Unspecified));
}

#[test]
fn high_with_value_and_null_flavor_keeps_the_value() {
    let observation = get_problem_observation(EFFECTIVE_TIME_FORMS, 6);
    let effective_time = observation.effective_time.unwrap();
    assert_eq!(effective_time.high, Some(TimePoint::Value("20140405".to_string())));
}

#[test]
fn author_time_null_flavor_is_captured() {
    let observation = get_problem_observation(EFFECTIVE_TIME_FORMS, 7);
    let time = observation.author.unwrap().time.unwrap();
    assert_eq!(time.null_flavor, Some("UNK".to_string()));
    assert_eq!(time.value, None);
}

#[test]
fn author_without_template_id_is_kept() {
    let observation = get_problem_observation(AUTHOR_WITHOUT_TEMPLATE_ID, 0);
    let author = observation
        .author
        .expect("author should be kept even without a templateId");
    assert!(author.template_id.is_none());
    assert!(author.time.is_some());
    assert!(author.assigned_author.is_some());
}

#[test]
fn date_of_diagnosis_act_nests_inside_observation_without_clobbering_outer_act() {
    let body = get_act_body(DATE_OF_DIAGNOSIS, 0);

    assert_eq!(body.id.unwrap().root.as_deref(), Some("outer-concern-act"));
    assert_eq!(body.status_code.as_deref(), Some("active"));

    let observation = body
        .entry_relationships
        .into_iter()
        .next()
        .and_then(|rel| rel.observation)
        .expect("problem observation should be attached to the outer act");

    let observation_low = observation.effective_time.as_ref().and_then(|t| t.low.as_ref());
    assert_eq!(observation_low, Some(&TimePoint::Value("20140227".to_string())));

    let diagnosis_act = observation
        .entry_relationships
        .into_iter()
        .next()
        .and_then(|rel| rel.act)
        .expect("date of diagnosis act should be attached to the observation");

    let diagnosis_body = diagnosis_act.act_body.unwrap();
    assert_eq!(
        diagnosis_body.id.unwrap().root.as_deref(),
        Some("date-of-diagnosis-act")
    );
    let low = diagnosis_body.effective_time.unwrap().low;
    assert_eq!(low, Some(TimePoint::Value("20140301".to_string())));
}

#[test]
fn act_inside_observation_keeps_its_own_entry_relationships() {
    let body = get_act_body(ACT_WITH_CHILDREN, 0);

    let observation = body
        .entry_relationships
        .into_iter()
        .next()
        .and_then(|rel| rel.observation)
        .expect("problem observation should be attached to the outer act");

    assert_eq!(
        observation.entry_relationships.len(),
        1,
        "only the inner act's relationship belongs to the observation"
    );

    let inner_act = observation
        .entry_relationships
        .into_iter()
        .next()
        .and_then(|rel| rel.act)
        .expect("inner act should be attached to the observation");

    let inner_observation = inner_act
        .act_body
        .unwrap()
        .entry_relationships
        .into_iter()
        .next()
        .and_then(|rel| rel.observation)
        .expect("inner act should own its own observation");

    assert_eq!(
        inner_observation.id.unwrap().root.as_deref(),
        Some("inner-observation")
    );
}

#[test]
fn section_text_keeps_the_narrative_markup() {
    let section = problem_section(RESOLVED_PROBLEM).expect("resolvedProblem.xml should parse");
    let text = section.text.expect("section text should be captured");

    assert!(text.contains("<table>"));
    assert!(text.contains(r#"<tr ID="Problem1">"#));
    assert!(text.contains("Acute pharyngitis"));
    assert!(text.contains("<br />"));
    assert!(
        !text.contains("<text>") && !text.contains("</text>"),
        "the wrapper tags are not part of the body"
    );
}
