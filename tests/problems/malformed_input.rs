use clinical_document_parser::parser::error::ParseError;
use clinical_document_parser::parser::problem_section;
use crate::fixtures::{MISSING_FILE, MISSING_END_TAG, NO_PROBLEMS_SECTION, EMPTY_FILE};

#[test]
fn missing_file_returns_io_error_with_the_path() {
    let err = problem_section(MISSING_FILE)
        .expect_err("parsing a missing file should fail");
    match err {
        ParseError::Io { path, .. } => assert_eq!(path, MISSING_FILE),
        other => panic!("expected ParseError::Io, got: {other}"),
    }
}

#[test]
fn missing_end_tag_returns_xml_error_with_position() {
    let err = problem_section(MISSING_END_TAG)
        .expect_err("missing end tag should fail");
    match err {
        ParseError::Xml { position, .. } => assert_eq!(position, 16),
        other => panic!("expected ParseError::Xml, got: {other}"),
    }
}

#[test]
fn document_without_problems_section_returns_no_problems_section_error() {
    let err = problem_section(NO_PROBLEMS_SECTION)
        .expect_err("a document without a Problems section should fail");
    match err {
        ParseError::NoProblemsSection { path } => assert_eq!(path.as_deref(), Some(NO_PROBLEMS_SECTION)),
        other => panic!("expected ParseError::NoProblemsSection, got: {other}"),
    }
}

#[test]
fn empty_file_returns_no_problems_section_error() {
    let err = problem_section(EMPTY_FILE)
        .expect_err("an empty file should fail");
    match err {
        ParseError::NoProblemsSection { path } => assert_eq!(path.as_deref(), Some(EMPTY_FILE)),
        other => panic!("expected ParseError::NoProblemsSection, got: {other}"),
    }
}
