use std::io::Write;
use std::process::{Command, Stdio};

mod common;

#[test]
fn public_key_mismatch() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_star_tools"))
        .args([
            "verify",
            "--mrsigner",
            &"02".repeat(32),
            "--public-key",
            &"09".repeat(32),
            "--pccs-url=http://127.0.0.1:1",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(&common::fake_quote())
        .unwrap();
    let output = child.wait_with_output().unwrap();

    assert_eq!(output.status.code(), Some(1));
    let error = String::from_utf8(output.stderr).unwrap();
    assert!(
        error.contains("attested public key does not match expected public key"),
        "{error}"
    );
    assert!(output.stdout.is_empty());
}
