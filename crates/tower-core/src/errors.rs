use thiserror::Error;

pub type Result<T> = std::result::Result<T, TowerError>;

#[derive(Debug, Error, PartialEq)]
pub enum TowerError {
    #[error("field `{field}` requires explicit unit `{expected}`")]
    MissingUnit { field: String, expected: String },

    #[error("field `{field}` has unit `{found}`, expected `{expected}`")]
    AmbiguousUnit {
        field: String,
        found: String,
        expected: String,
    },

    #[error("field `{field}` must be finite, got {value}")]
    NonFiniteValue { field: String, value: f64 },

    #[error("field `{field}` must be greater than zero, got {value}")]
    NonPositiveValue { field: String, value: f64 },

    #[error("field `{field}` must be zero or greater, got {value}")]
    NegativeValue { field: String, value: f64 },

    #[error("duplicate id `{id}` in `{collection}`")]
    DuplicateId { collection: String, id: String },

    #[error("unknown reference `{id}` in field `{field}`")]
    UnknownReference { field: String, id: String },

    #[error("input parsing failed: {message}")]
    Parse { message: String },

    #[error("{feature} is blocked until domain validation is complete")]
    BlockedDomainFeature { feature: &'static str },

    #[error("model is invalid for analysis: {reason}")]
    InvalidAnalysisModel { reason: String },

    #[error("model is unstable: {reason}")]
    UnstableModel { reason: String },
}
