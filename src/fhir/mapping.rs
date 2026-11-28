// OID -> FHIR system URI pairs, each checked against an official source:
// hl7.org/fhir/R4/terminologies-systems.html, hl7.org/fhir/R4/icd.html,
// and the BfArM terminology server (ICD-10-GM).
pub fn oid_to_uri(oid: &str) -> String {
    match oid {
        "2.16.840.1.113883.6.96" => "http://snomed.info/sct".to_string(),
        "2.16.840.1.113883.6.1" => "http://loinc.org".to_string(),
        "2.16.840.1.113883.6.3" => "http://hl7.org/fhir/sid/icd-10".to_string(),
        "2.16.840.1.113883.6.90" => "http://hl7.org/fhir/sid/icd-10-cm".to_string(),
        "1.2.276.0.76.5.554" => "http://fhir.de/CodeSystem/bfarm/icd-10-gm".to_string(),
        other => format!("urn:oid:{other}"),
    }
}

pub fn hl7_time_to_fhir(value: &str) -> Option<String> {
    if !value.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    match value.len() {
        4 => Some(value.to_string()),
        6 => Some(format!("{}-{}", &value[0..4], &value[4..6])),
        8 => Some(format!("{}-{}-{}", &value[0..4], &value[4..6], &value[6..8])),
        _ => None,
    }
}
