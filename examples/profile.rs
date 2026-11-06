use clinical_document_parser::parser::problem_section_from_reader;

fn main() {
    let bytes = std::fs::read("sample-documents/generated/medium_5mb.xml")
        .expect("generate the file first");
    for _ in 0..400 {
        std::hint::black_box(problem_section_from_reader(bytes.as_slice()).unwrap());
    }
}
