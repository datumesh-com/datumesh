//! `datumesh key rotate` and `datumesh root` key refusal (CONTRACT-1.8b L4, L5).

use std::process::Command;

fn home(what: &str) -> std::path::PathBuf {
    let p = std::env::temp_dir().join(format!("datumesh-keys-cli-{what}-{}", std::process::id()));
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

/// The string value of `"field":"<16 hex>"` in a compact JSON text.
fn key_id(v: &str, field: &str) -> String {
    let pat = format!(r#""{field}":""#);
    let i = v.find(&pat).unwrap_or_else(|| panic!("{field} in {v}")) + pat.len();
    let s = v[i..].split('"').next().unwrap().to_string();
    assert_eq!(s.len(), 16, "{s}");
    assert!(s.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)), "{s}");
    s
}

#[test]
fn key_rotate_prints_old_and_new_and_the_document_lists_two_keys() {
    let h = home("rotate");
    let (code, _, err) = datumesh(&h, &["key"]);
    assert_eq!(code, 0, "{err}");
    let (code, out, err) = datumesh(&h, &["key", "rotate"]);
    assert_eq!(code, 0, "{err}");
    let old = key_id(&out, "old_key_id");
    let new = key_id(&out, "new_key_id");
    assert_ne!(old, new);
    let (code, out, err) = datumesh(&h, &["key"]);
    assert_eq!(code, 0, "{err}");
    assert_eq!(out.matches(r#""public""#).count(), 2, "{out}");
    assert!(out.contains(&old) && out.contains(&new), "{out}");
    let _ = std::fs::remove_dir_all(&h);
}

#[cfg(unix)]
#[test]
fn root_refuses_a_world_readable_key_file() {
    use std::os::unix::fs::PermissionsExt;
    let h = home("root-mode");
    let key = h.join("ns.key");
    std::fs::write(&key, format!("{}\n", "ab".repeat(32))).unwrap();
    std::fs::set_permissions(&key, std::fs::Permissions::from_mode(0o644)).unwrap();
    let (code, out, err) = datumesh(
        &h,
        &["root", "--namespace", "mesh-test.datumesh.com/scratch_1_8b", "--key", key.to_str().unwrap(), "local/x"],
    );
    assert_ne!(code, 0, "{err}");
    assert!(out.is_empty(), "stdout: {out}");
    assert!(!err.is_empty());
    let _ = std::fs::remove_dir_all(&h);
}
