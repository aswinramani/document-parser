use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("cannot open {path}: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("malformed XML at byte {position}: {source}")]
    Xml {
        position: u64,
        #[source]
        source: quick_xml::Error,
    },
    #[error("no Problems section (LOINC 11450-4) found in {}", path.as_deref().unwrap_or("<input>"))]
    NoProblemsSection { path: Option<String> },

}
