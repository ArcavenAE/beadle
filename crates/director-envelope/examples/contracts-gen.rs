//! `just contracts-gen`: regenerate src/envelope.gen.rs from the vendored schema.

#[path = "../codegen/mod.rs"]
mod codegen;

use std::{fs, path::Path};

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let schema = fs::read_to_string(root.join("contracts/director-envelope.schema.json"))
        .expect("read vendored schema");
    let out = root.join("src/envelope.gen.rs");
    fs::write(&out, codegen::generate(&schema)).expect("write generated types");
    println!("contracts-gen: regenerated {}", out.display());
}
