use std::process::{Command, Output};

fn run(home: &std::path::Path, args: &[&str]) -> Output {
    let output = Command::new(assert_cmd::cargo::cargo_bin!("fnord"))
        .env_clear()
        .env("HOME", home)
        .env("XDG_CONFIG_HOME", home.join("config"))
        .current_dir(home)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

#[test]
fn color_changes_only_ansi_styling_of_diff_output() {
    let temp = tempfile::tempdir().unwrap();
    std::fs::write(temp.path().join("a"), "b\na\na\n").unwrap();
    std::fs::write(temp.path().join("b"), "c\na\nc\n").unwrap();
    let colored = run(temp.path(), &["erisian", "a", "b", "--no-unicode"]);
    let plain = run(
        temp.path(),
        &["erisian", "a", "b", "--no-unicode", "--no-color"],
    );
    assert!(colored.stdout.contains(&0x1b));
    assert!(!plain.stdout.contains(&0x1b));
    let mut stripped = Vec::new();
    let mut bytes = colored.stdout.into_iter();
    while let Some(byte) = bytes.next() {
        if byte == 0x1b {
            assert_eq!(bytes.next(), Some(b'['));
            for code in bytes.by_ref() {
                if (0x40..=0x7e).contains(&code) {
                    break;
                }
            }
        } else {
            stripped.push(byte);
        }
    }
    assert_eq!(stripped, plain.stdout);
}

#[cfg(target_os = "linux")]
#[test]
fn personal_toml_strings_survive_json_output() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().join("config/eris/holydays");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("personal.toml"),
        r#"
[[holyday]]
date = "chaos-42"
name = 'Eris "quoted" ☯'
description = "line one\nline two\\path"
"#,
    )
    .unwrap();
    let output = run(temp.path(), &["holyday", "list", "--json"]);
    assert!(!output.stdout.contains(&0x1b));
    let rows: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let row = rows
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["key"] == "chaos-42")
        .unwrap();
    assert_eq!(row["name"], "Eris \"quoted\" ☯");
    assert_eq!(row["description"], "line one\nline two\\path");
    assert_eq!(row["source"], "personal");
}
