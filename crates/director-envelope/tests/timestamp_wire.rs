//! The three timestamp fields of the director envelope, pinned as chrono reads
//! and writes them today (`sent_at`, `expires_at`, `reply_by`, each a
//! `chrono::DateTime<Utc>` in the generated types). chrono is flagged unmaintained
//! (RUSTSEC-2026-0543), and a replacement time crate must not change what an
//! envelope looks like on the bus: these values were measured against chrono
//! 0.4.45 and are the contract the swap has to keep.
//!
//! Output: RFC 3339 in UTC with a `Z` suffix, whole seconds when the fraction is
//! zero, otherwise exactly 3, 6 or 9 fractional digits (the shortest of the three
//! that holds the value), never a numeric offset. Input: an offset is converted to
//! UTC; `z` and `t` in lower case and a space in place of `T` are accepted; a
//! seconds field of 60 (a leap second) is accepted and written back as `:60`;
//! a date alone, a missing offset, a missing seconds field, a comma as the
//! decimal mark, and out-of-range fields are refused.

use director_envelope::Envelope;
use serde_json::{json, Value};

const FIELDS: [&str; 3] = ["sent_at", "expires_at", "reply_by"];

/// (input, Some(output) when accepted, None when refused)
const CASES: &[(&str, Option<&str>)] = &[
    // output shape: whole seconds, then 3, 6 or 9 digits
    ("2026-11-18T15:52:42Z", Some("2026-11-18T15:52:42Z")),
    ("2026-11-18T15:52:42.5Z", Some("2026-11-18T15:52:42.500Z")),
    ("2026-11-18T15:52:42.50Z", Some("2026-11-18T15:52:42.500Z")),
    ("2026-11-18T15:52:42.282Z", Some("2026-11-18T15:52:42.282Z")),
    (
        "2026-11-18T15:52:42.2820Z",
        Some("2026-11-18T15:52:42.282Z"),
    ),
    (
        "2026-11-18T15:52:42.282010Z",
        Some("2026-11-18T15:52:42.282010Z"),
    ),
    (
        "2026-11-18T15:52:42.28201Z",
        Some("2026-11-18T15:52:42.282010Z"),
    ),
    (
        "2026-11-18T15:52:42.282010123Z",
        Some("2026-11-18T15:52:42.282010123Z"),
    ),
    (
        "2026-11-18T15:52:42.1234567890Z",
        Some("2026-11-18T15:52:42.123456789Z"),
    ),
    // offsets become UTC, and the output never carries one
    ("2026-11-18T15:52:42+00:00", Some("2026-11-18T15:52:42Z")),
    ("2026-11-18T10:22:42-05:30", Some("2026-11-18T15:52:42Z")),
    // lenient separators and case
    ("2026-11-18T15:52:42z", Some("2026-11-18T15:52:42Z")),
    ("2026-11-18t15:52:42Z", Some("2026-11-18T15:52:42Z")),
    ("2026-11-18 15:52:42Z", Some("2026-11-18T15:52:42Z")),
    // a leap second is kept
    ("2026-11-18T23:59:60Z", Some("2026-11-18T23:59:60Z")),
    // the ends of the supported years
    ("0001-01-01T00:00:00Z", Some("0001-01-01T00:00:00Z")),
    ("9999-12-31T23:59:59Z", Some("9999-12-31T23:59:59Z")),
    // refused
    ("2026-11-18T15:52:42", None),
    ("2026-11-18", None),
    ("2026-11-18T15:52Z", None),
    ("2026-11-18T15:52:42,5Z", None),
    ("2026-02-30T00:00:00Z", None),
    ("2026-11-18T24:00:00Z", None),
];

fn base() -> Value {
    serde_json::from_str(include_str!("../contracts/testdata/valid-minimal.json")).unwrap()
}

fn run(field: &str, input: &str) -> Result<String, String> {
    let mut v = base();
    v[field] = Value::String(input.to_string());
    let env: Envelope = serde_json::from_value(v).map_err(|e| e.to_string())?;
    let out = serde_json::to_value(&env).unwrap();
    Ok(out[field]
        .as_str()
        .expect("the field is a string")
        .to_string())
}

#[test]
fn every_timestamp_field_reads_and_writes_the_chrono_shape() {
    for field in FIELDS {
        for (input, want) in CASES {
            let got = run(field, input);
            match want {
                Some(w) => assert_eq!(got.as_deref(), Ok(*w), "{field}: {input}"),
                None => assert!(
                    got.is_err(),
                    "{field}: {input} should be refused, got {got:?}"
                ),
            }
        }
    }
}

#[test]
fn a_written_timestamp_reads_back_to_the_same_text() {
    for field in FIELDS {
        for (_, want) in CASES {
            if let Some(w) = want {
                assert_eq!(
                    run(field, w).as_deref(),
                    Ok(*w),
                    "{field}: {w} is not stable"
                );
            }
        }
    }
}

#[test]
fn an_absent_optional_timestamp_stays_absent_and_the_required_one_is_required() {
    let env: Envelope = serde_json::from_value(base()).unwrap();
    let out = serde_json::to_value(&env).unwrap();
    assert!(out.get("expires_at").is_none());
    assert!(out.get("reply_by").is_none());
    assert_eq!(out["sent_at"], json!("2026-09-12T23:40:00Z"));

    let mut v = base();
    v.as_object_mut().unwrap().remove("sent_at");
    assert!(serde_json::from_value::<Envelope>(v).is_err());

    let mut v = base();
    v["sent_at"] = Value::Null;
    assert!(serde_json::from_value::<Envelope>(v).is_err());
}

#[test]
fn a_timestamp_that_is_not_a_string_is_refused() {
    for field in FIELDS {
        for bad in [
            json!(1_700_000_000),
            json!(1.5),
            json!(true),
            json!([]),
            json!({}),
        ] {
            let mut v = base();
            v[field] = bad.clone();
            assert!(
                serde_json::from_value::<Envelope>(v).is_err(),
                "{field}: {bad} should be refused"
            );
        }
    }
}
