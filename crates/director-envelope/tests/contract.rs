//! The contract tests, mirroring marvel's contracts/go/envelope/envelope_test.go
//! so both consumers prove the same things against the same schema and the
//! same fixtures.

#[path = "../codegen/mod.rs"]
mod codegen;

use std::{fs, path::PathBuf};

use director_envelope::{
    validate, DirectorEnvelopeAuthorityStrength, DirectorEnvelopeContentType,
    DirectorEnvelopePerformative, Envelope, SCHEMA_JSON,
};
use sha2::{Digest, Sha256};

fn contracts_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("contracts")
}

fn fixtures(prefix: &str) -> Vec<(String, Vec<u8>)> {
    let mut out: Vec<(String, Vec<u8>)> = fs::read_dir(contracts_dir().join("testdata"))
        .expect("read testdata")
        .map(|e| e.expect("dir entry").path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with(prefix) && n.ends_with(".json"))
        })
        .map(|p| {
            let name = p.file_name().unwrap().to_string_lossy().into_owned();
            (name, fs::read(&p).expect("read fixture"))
        })
        .collect();
    out.sort();
    assert!(!out.is_empty(), "no {prefix}* fixtures found");
    out
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

/// The vendored copy is a pin, not a fork: every file under contracts/ must
/// hash to the value recorded in PINNED.sha256, so a hand edit cannot pass as
/// the canonical schema and a refresh names the upstream revision it took.
#[test]
fn vendored_contract_matches_pin() {
    let pins =
        fs::read_to_string(contracts_dir().join("PINNED.sha256")).expect("read PINNED.sha256");
    let mut checked = 0;
    for line in pins.lines().filter(|l| !l.trim().is_empty()) {
        let (want, rel) = line
            .split_once("  ")
            .unwrap_or_else(|| panic!("PINNED.sha256 line has no '  ' separator: {line:?}"));
        let bytes =
            fs::read(contracts_dir().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"));
        let got = sha256_hex(&bytes);
        assert_eq!(
            got, want,
            "{rel} drifted from contracts/PINNED.sha256; refresh the pin from marvel \
             (contracts/PINNED.md) rather than editing the copy"
        );
        checked += 1;
    }
    assert_eq!(
        checked, 12,
        "PINNED.sha256 should list the schema plus 11 fixtures"
    );
    assert_eq!(
        sha256_hex(SCHEMA_JSON.as_bytes()),
        "101ce10196f156243a20d8a5e22b07433869286b08a993a3726d9ed06ae127ad",
        "the schema the validator embeds is not the pinned one"
    );
}

/// When a marvel checkout sits beside this one (or MARVEL_CONTRACTS_DIR names
/// its contracts/ directory), also prove the pin equals the canonical bytes.
/// Skipped, not failed, when marvel is absent: beadle builds alone.
#[test]
fn vendored_contract_matches_canonical_when_present() {
    let canonical = std::env::var_os("MARVEL_CONTRACTS_DIR")
        .map(PathBuf::from)
        .or_else(|| {
            let sibling =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../marvel/contracts");
            sibling.is_dir().then_some(sibling)
        });
    let Some(canonical) = canonical else {
        eprintln!("marvel contracts/ not found; skipping the canonical byte comparison");
        return;
    };
    let theirs = fs::read(canonical.join("schema/director-envelope.schema.json"))
        .expect("read canonical schema");
    assert_eq!(
        String::from_utf8_lossy(&theirs),
        SCHEMA_JSON,
        "vendored schema differs from {}; refresh the pin",
        canonical.display()
    );
}

/// envelope.gen.rs is what the generator emits for the vendored schema; a
/// schema refresh that skips `just contracts-gen` fails here rather than
/// shipping stale types.
#[test]
fn generated_types_match_regeneration() {
    let checked_in =
        fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/envelope.gen.rs"))
            .expect("read envelope.gen.rs");
    let fresh = codegen::generate(SCHEMA_JSON);
    assert_eq!(
        checked_in, fresh,
        "src/envelope.gen.rs drifted from the schema; run `just contracts-gen`"
    );
}

#[test]
fn valid_fixtures_validate() {
    for (name, data) in fixtures("valid-") {
        if let Err(e) = validate(&data) {
            panic!("{name}: expected valid, got: {e}");
        }
    }
}

#[test]
fn invalid_fixtures_rejected() {
    for (name, data) in fixtures("invalid-") {
        assert!(
            validate(&data).is_err(),
            "{name}: expected rejection, but validation passed"
        );
    }
}

/// The generated types decode a real envelope, the typed enums carry the wire
/// values, and a serialization of the result still validates.
#[test]
fn round_trip() {
    let data =
        fs::read(contracts_dir().join("testdata/valid-principal-null.json")).expect("read fixture");
    let e: Envelope = serde_json::from_slice(&data).expect("deserialize into Envelope");
    assert!(
        matches!(e.performative, DirectorEnvelopePerformative::Request),
        "performative: got {:?}",
        e.performative
    );
    assert!(matches!(
        e.authority.strength,
        DirectorEnvelopeAuthorityStrength::Direct
    ));
    assert!(matches!(e.content.type_, DirectorEnvelopeContentType::Task));
    let out = serde_json::to_vec(&e).expect("serialize Envelope");
    if let Err(err) = validate(&out) {
        panic!("re-validate after round-trip: {err}");
    }
}

/// Every valid fixture also decodes into the generated types, and every one of
/// them survives the round trip; the types and the validator agree on the
/// accepted set.
#[test]
fn valid_fixtures_decode_and_round_trip() {
    for (name, data) in fixtures("valid-") {
        let e: Envelope =
            serde_json::from_slice(&data).unwrap_or_else(|err| panic!("{name}: decode: {err}"));
        let out = serde_json::to_vec(&e).expect("serialize");
        if let Err(err) = validate(&out) {
            panic!("{name}: re-validate after round-trip: {err}");
        }
    }
}
