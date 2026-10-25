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
}
