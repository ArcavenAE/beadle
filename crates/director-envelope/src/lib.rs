//! director-envelope: the Rust types and validator for the director envelope
//! contract (bd aae-orc-b69n).
//!
//! The single source of truth is the JSON Schema in the marvel repository,
//! vendored byte-identical at `contracts/director-envelope.schema.json` and
//! pinned by sha256 (`contracts/PINNED.md`). `envelope.gen.rs` (the types) is
//! generated from it by `just contracts-gen`, and the validator compiles the
//! same file at first use. marvel's Go package `contracts/go/envelope`
//! generates from the same schema, so the two consumers cannot drift; a test
//! here guards the copy against the pin, and one there guards the embedded
//! copy against the canonical.

use std::sync::LazyLock;

#[rustfmt::skip]
#[allow(clippy::all, missing_docs)]
mod envelope_gen {
    include!("envelope.gen.rs");
}

pub use envelope_gen::*;

/// The director envelope. Aliases the generated root type so callers write
/// `director_envelope::Envelope` rather than the generator's derived name. The
/// alias is the one hand-written type name here; everything else is generated.
pub type Envelope = DirectorEnvelope;

/// The envelope JSON Schema, embedded so the validator needs no filesystem at
/// runtime. This is the vendored copy of the canonical schema; the pin guard
/// test keeps it honest.
pub const SCHEMA_JSON: &str = include_str!("../contracts/director-envelope.schema.json");

/// The schema's `$id`. The compiler resolves the document against its own
/// identifier rather than fetching the URL.
pub const SCHEMA_ID: &str = "https://schema.arcaven.com/director/envelope/v1";

static COMPILED: LazyLock<jsonschema::Validator> = LazyLock::new(|| {
    let schema: serde_json::Value = serde_json::from_str(SCHEMA_JSON)
        .expect("director-envelope: embedded schema is not valid JSON");
    jsonschema::options()
        .with_draft(jsonschema::Draft::Draft202012)
        .build(&schema)
        .expect("director-envelope: compile schema")
});

/// A validation failure: the instance did not decode as JSON, or the schema
/// rejected it. Every schema violation is listed, not only the first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidateError {
    /// The bytes are not JSON.
    Decode(String),
    /// The JSON is not a director envelope; one entry per violation, each as
    /// `<instance path>: <message>`.
    Schema(Vec<String>),
}

impl std::fmt::Display for ValidateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Decode(e) => write!(f, "envelope: decode instance: {e}"),
            Self::Schema(errs) => {
                write!(f, "envelope: {} schema violation(s)", errs.len())?;
                for e in errs {
                    write!(f, "\n  {e}")?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for ValidateError {}

/// Report whether raw JSON is a valid director envelope. The schema enforces
/// the identity and authority shape by pattern and enum, so format checks hold
/// without a format-assertion mode (the reason the schema carries patterns
/// beside its format annotations).
pub fn validate(data: &[u8]) -> Result<(), ValidateError> {
    let instance: serde_json::Value =
        serde_json::from_slice(data).map_err(|e| ValidateError::Decode(e.to_string()))?;
    validate_value(&instance)
}

/// [`validate`] for an already-decoded JSON value.
pub fn validate_value(instance: &serde_json::Value) -> Result<(), ValidateError> {
    let errors: Vec<String> = COMPILED
        .iter_errors(instance)
        .map(|e| format!("{}: {}", e.instance_path(), e))
        .collect();
    if errors.is_empty() {
        Ok(())
    } else {
        Err(ValidateError::Schema(errors))
    }
}
