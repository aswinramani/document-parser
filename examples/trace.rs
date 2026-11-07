use clinical_document_parser::parser::problem_section;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    problem_section("sample-documents/problems/nestedRelationships.xml").unwrap();
}
