//! The binary over MCP stdio (CONTRACT-1.8b L1, L2).

use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

fn home(what: &str) -> std::path::PathBuf {
    let p = std::env::temp_dir().join(format!("datumesh-mcp-test-{what}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&p);
    std::fs::create_dir_all(&p).unwrap();
    p
}

/// Spawns `datumesh mcp`, sends each line, reads `expect` response lines, closes stdin and
/// waits for exit 0 within 5 s. Returns the parsed response lines; stdout must hold no more.
fn session(what: &str, lines: &[&str], expect: usize) -> Vec<String> {
    let h = home(what);
    let mut child = Command::new(env!("CARGO_BIN_EXE_datumesh"))
        .arg("--home")
        .arg(&h)
        .arg("mcp")
        .env_remove("DATUMESH_HOME")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = BufReader::new(child.stdout.take().unwrap());
    for l in lines {
        writeln!(stdin, "{l}").unwrap();
        stdin.flush().unwrap();
    }
    let mut got = Vec::new();
    for _ in 0..expect {
        let mut s = String::new();
        assert!(stdout.read_line(&mut s).unwrap() > 0, "stdout closed early");
        assert!(s.starts_with('{') && s.trim_end().ends_with('}'), "not a JSON object line: {s}");
        got.push(s);
    }
    drop(stdin);
    let start = Instant::now();
    let status = loop {
        if let Some(s) = child.try_wait().unwrap() {
            break s;
        }
        if start.elapsed() > Duration::from_secs(5) {
            let _ = child.kill();
            panic!("datumesh mcp did not exit within 5 s of EOF");
        }
        std::thread::sleep(Duration::from_millis(20));
    };
    assert!(status.success(), "{status:?}");
    let mut rest = String::new();
    std::io::Read::read_to_string(&mut stdout, &mut rest).unwrap();
    assert!(rest.trim().is_empty(), "extra stdout: {rest}");
    let _ = std::fs::remove_dir_all(&h);
    got
}

const META: &str = r#""_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientCapabilities":{}}"#;

/// Checks the tool names occur, in order, as `"name":"<tool>"` (no JSON dependency here).
fn assert_tools_in_order(v: &str) {
    let mut at = 0;
    for t in TOOLS {
        let pat = format!(r#""name":"{t}""#);
        let i = v[at..].find(&pat).unwrap_or_else(|| panic!("tool {t} missing or out of order in {v}"));
        at += i + pat.len();
    }
    assert_eq!(v.matches(r#""inputSchema""#).count(), TOOLS.len(), "{v}");
}

const TOOLS: [&str; 8] = ["find", "get", "explain", "describe", "verify", "define", "publish", "subscribe"];

#[test]
fn modern_discover_and_tools_list() {
    let discover = format!(r#"{{"jsonrpc":"2.0","id":1,"method":"server/discover","params":{{{META}}}}}"#);
    let list = format!(r#"{{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{{{META}}}}}"#);
    let got = session("modern", &[&discover, &list], 2);
    assert!(got[0].contains(r#""id":1"#), "{}", got[0]);
    assert!(got[0].contains(r#""supportedVersions":["2026-07-28"]"#), "{}", got[0]);
    assert!(got[1].contains(r#""id":2"#), "{}", got[1]);
    assert_tools_in_order(&got[1]);
}

#[test]
fn legacy_handshake() {
    let init = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"legacy-client","version":"1.0"}}}"#;
    let inited = r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#;
    let list = r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#;
    let got = session("legacy", &[init, inited, list], 2);
    assert!(got[0].contains(r#""id":1"#), "{}", got[0]);
    assert!(got[0].contains(r#""protocolVersion":"2025-06-18""#), "{}", got[0]);
    assert!(got[1].contains(r#""id":2"#), "{}", got[1]);
    assert_tools_in_order(&got[1]);
}

#[test]
fn http_refuses_a_non_loopback_listen_address() {
    let h = home("http-listen");
    let out = Command::new(env!("CARGO_BIN_EXE_datumesh"))
        .arg("--home")
        .arg(&h)
        .args(["mcp", "--http", "--listen", "0.0.0.0:7799"])
        .env_remove("DATUMESH_HOME")
        .output()
        .unwrap();
    assert!(!out.status.success());
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(err.to_lowercase().contains("loopback"), "{err}");
    let _ = std::fs::remove_dir_all(&h);
}
