//! The binary with classes (spec-0.5.0; `CONTRACT-local-0.5.0.md` items 1, 2 and 5): `describe`,
//! `define --class`, a method call in `get` with the canonical call on stderr, all offline over a
//! local class (no mesh is reached).

use std::process::Command;

fn home(what: &str) -> std::path::PathBuf {
    let p = std::env::temp_dir().join(format!("datumesh-bin-classes-{what}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&p);
    std::fs::create_dir_all(&p).unwrap();
    p
}

fn datumesh(home: &std::path::Path, args: &[&str]) -> (i32, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_datumesh"))
        .arg("--home")
        .arg(home)
        .arg("--offline")
        .args(args)
        .env_remove("DATUMESH_HOME")
        .output()
        .unwrap();
    (out.status.code().unwrap_or(-1), String::from_utf8(out.stdout).unwrap(), String::from_utf8(out.stderr).unwrap())
}

const RATE_SIG: &str = r#"{"args": {"base": {"type": "string", "valid": false}, "date": {"type": "date", "valid": true}, "quote": {"type": "string", "valid": false}}, "order": ["base", "quote", "date"], "returns": {"type": "number", "unit": "1"}}"#;

fn json_member(text: &str, key: &str) -> String {
    let at = text.find(&format!("\"{key}\":\"")).unwrap_or_else(|| panic!("no {key} in {text}"));
    let rest = &text[at + key.len() + 4..];
    rest[..rest.find('"').unwrap()].to_string()
}

#[test]
fn the_usage_lists_describe_and_define_class() {
    let h = home("usage");
    let (code, out, _) = datumesh(&h, &["--help"]);
    assert_eq!(code, 0);
    assert!(out.contains("describe"), "{out}");
    assert!(out.contains("--class"), "{out}");
    let _ = std::fs::remove_dir_all(&h);
}

#[test]
fn a_local_class_is_defined_described_and_called() {
    let h = home("local");
    let (code, out, err) = datumesh(&h, &["define", "local/pair/rate", "--stored", "--signature", RATE_SIG]);
    assert_eq!(code, 0, "{err}");
    let hash = json_member(&out, "fn_hash");
    let class = h.join("pair.json");
    std::fs::write(
        &class,
        format!(
            r#"{{"keys": {{"base": {{"key_normalisation": "none", "type": "string"}}, "quote": {{"key_normalisation": "none", "type": "string"}}}}, "methods": {{"rate": "{hash}"}}, "order": ["base", "quote"], "steward": {{"namespace": "local.invalid", "process": "https://local.invalid/classes/change"}}}}"#
        ),
    )
    .unwrap();
    let (code, out, err) = datumesh(&h, &["define", "local/pair", "--class", class.to_str().unwrap()]);
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("local.invalid/pair"), "{out}");
    // describe, offline, from the local class.
    let (code, out, err) = datumesh(&h, &["describe", "local/pair"]);
    assert_eq!(code, 0, "{err}");
    assert_eq!(json_member(&out, "kind"), "class", "{out}");
    let (code, obj, err) = datumesh(&h, &["describe", r#"local.invalid/pair(base:"EUR",quote:"USD")"#]);
    assert_eq!(code, 0, "{err}");
    let short = json_member(&obj, "short_id");
    let (code, by_short, err) = datumesh(&h, &["describe", &short]);
    assert_eq!(code, 0, "{err}");
    assert_eq!(by_short, obj);
    // A cell, then the method call: the envelope has `resolved`, the stderr line the canonical call.
    let cells = h.join("cells.jsonl");
    std::fs::write(
        &cells,
        concat!(
            r#"{"cell": {"fn": "local/pair/rate", "args": {"base": {"type": "string", "value": "EUR"}, "date": {"type": "date", "value": "2026-03-02"}, "quote": {"type": "string", "value": "USD"}}}, "value": {"type": "number", "value": "1.2"}, "tx": "2026-03-02T16:00:00.000000Z", "asof": "2026-03-02T16:00:00.000000Z", "licence": "CC0-1.0", "summary": "A rate."}"#,
            "\n"
        ),
    )
    .unwrap();
    let (code, _, err) = datumesh(&h, &["publish", cells.to_str().unwrap()]);
    assert_eq!(code, 0, "{err}");
    let (code, out, err) = datumesh(&h, &["get", "local/pair(EUR, USD).rate(2026-03-02)"]);
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("\"resolved\":{"), "{out}");
    let canonical = r#"local.invalid/pair/rate(base: "EUR", date: 2026-03-02, quote: "USD")"#;
    assert!(err.contains(canonical), "the stderr line shows the canonical call: {err}");
    let (code, plain, err) = datumesh(&h, &["get", r#"local/pair/rate(base: "EUR", quote: "USD", date: 2026-03-02)"#]);
    assert_eq!(code, 0, "{err}");
    assert!(!plain.contains("\"resolved\""), "{plain}");
    assert_eq!(json_member(&plain, "hash"), json_member(&out, "hash"), "the same receipt");
    let _ = std::fs::remove_dir_all(&h);
}
