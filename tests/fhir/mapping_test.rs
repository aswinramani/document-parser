use clinical_document_parser::fhir::mapping::{oid_to_uri, hl7_time_to_fhir, clinical_status_from_act_status};

#[test]
fn known_oids_map_to_their_fhir_uris() {
    assert_eq!(oid_to_uri("2.16.840.1.113883.6.96"), "http://snomed.info/sct");
    assert_eq!(oid_to_uri("2.16.840.1.113883.6.1"), "http://loinc.org");
}

#[test]
fn unknown_oid_becomes_a_urn() {
    assert_eq!(oid_to_uri("1.2.3.4"), "urn:oid:1.2.3.4");
}

#[test]
fn icd_10_variants_map_to_their_fhir_uris() {
    assert_eq!(oid_to_uri("2.16.840.1.113883.6.3"), "http://hl7.org/fhir/sid/icd-10");
    assert_eq!(oid_to_uri("2.16.840.1.113883.6.90"), "http://hl7.org/fhir/sid/icd-10-cm");
    assert_eq!(
        oid_to_uri("1.2.276.0.76.5.554"),
        "http://fhir.de/CodeSystem/bfarm/icd-10-gm"
    );
}

#[test]
fn date_forms_keep_their_precision() {
    assert_eq!(hl7_time_to_fhir("2014"), Some("2014".to_string()));
    assert_eq!(hl7_time_to_fhir("201402"), Some("2014-02".to_string()));
    assert_eq!(hl7_time_to_fhir("20140227"), Some("2014-02-27".to_string()));
}

#[test]
fn unsupported_input_returns_none() {
    assert_eq!(hl7_time_to_fhir(""), None);
    assert_eq!(hl7_time_to_fhir("20140"), None);
    assert_eq!(hl7_time_to_fhir("2014-02"), None);
}

#[test]
fn active_act_status_becomes_active() {
    let status = clinical_status_from_act_status("active").unwrap();
    assert_eq!(status.coding[0].code, Some("active".to_string()));
}

#[test]
fn finished_act_statuses_become_inactive() {
    for s in ["completed", "aborted", "suspended"] {
        let status = clinical_status_from_act_status(s).unwrap();
        assert_eq!(status.coding[0].code, Some("inactive".to_string()));
    }
}

#[test]
fn unknown_act_status_returns_none() {
    assert_eq!(clinical_status_from_act_status("held"), None);
}
