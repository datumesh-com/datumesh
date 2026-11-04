//! The binary: `cargo install datumesh` gives the `datumesh` command of datumesh-local's CLI.

use std::process::Command;

fn home(what: &str) -> std::path::PathBuf {
    let p = std::env::temp_dir().join(format!("datumesh-bin-test-{what}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&p);
    std::fs::create_dir_all(&p).unwrap();
    p
}

fn datumesh(home: &std::path::Path, args: &[&str]) -> (i32, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_datumesh"))
        .arg("--home")
        .arg(home)
        .args(args)
        .env_remove("DATUMESH_HOME")
        .output()
        .unwrap();
    (out.status.code().unwrap_or(-1), String::from_utf8(out.stdout).unwrap(), String::from_utf8(out.stderr).unwrap())
}

#[test]
fn version_and_help() {
    let h = home("version");
    let (code, out, _) = datumesh(&h, &["--version"]);
    assert_eq!(code, 0);
    assert!(out.starts_with("datumesh "), "{out}");
    let (code, out, _) = datumesh(&h, &["--help"]);
    assert_eq!(code, 0);
    assert!(out.contains("prefetch"), "{out}");
    assert_eq!(datumesh(&h, &["nonsense"]).0, 2);
    let _ = std::fs::remove_dir_all(&h);
}

#[test]
fn first_run_offline_key_and_an_unfetched_cell() {
    let h = home("first-run");
    let (code, out, err) = datumesh(&h, &["key"]);
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("\"machine\":\""), "{out}");
    assert!(out.contains("local.invalid"), "{out}");
    for f in ["config.json", "aliases.json", "machine.key", "store.sqlite", "index.sqlite"] {
        assert!(h.join(f).exists(), "{f}");
    }
    let (code, out, err) =
        datumesh(&h, &["--offline", "get", r#"ecb/fx_rate(base: "EUR", quote: "USD", date: 2026-10-30)"#]);
    assert_eq!(code, 3, "{err}");
    assert_eq!(out.trim_end(), r#"{"detail":"offline","error":"E_FOREIGN_UNAVAILABLE","outcome":"error"}"#);
    assert!(err.contains("offline"), "{err}");
    let (code, _, err) = datumesh(&h, &["serve", "--listen", "0.0.0.0:0"]);
    assert_eq!(code, 1);
    assert!(err.contains("127.0.0.1 only"), "{err}");
    let _ = std::fs::remove_dir_all(&h);
}
