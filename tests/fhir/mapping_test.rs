use clinical_document_parser::fhir::mapping::oid_to_uri;

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
