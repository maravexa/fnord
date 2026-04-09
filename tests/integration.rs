use assert_cmd::Command;
use predicates::prelude::*;

fn fnord() -> Command {
    Command::cargo_bin("fnord").unwrap()
}

#[test]
fn test_no_args_exits_zero_and_prints_date() {
    fnord()
        .assert()
        .success()
        .stdout(predicate::str::contains("YOLD"));
}

#[test]
fn test_date_subcommand_today() {
    fnord()
        .arg("date")
        .assert()
        .success()
        .stdout(predicate::str::contains("YOLD"));
}

#[test]
fn test_date_mungday() {
    fnord()
        .args(["date", "--date", "2025-01-05"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Mungday"));
}

#[test]
fn test_date_st_tibs() {
    fnord()
        .args(["date", "--date", "2024-02-29"])
        .assert()
        .success()
        .stdout(predicate::str::contains("St. Tib's Day"));
}

#[test]
fn test_date_short_exits_zero() {
    fnord()
        .args(["date", "--short"])
        .assert()
        .success();
}

#[test]
fn test_cal_exits_zero() {
    fnord()
        .arg("cal")
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_help_lists_subcommands() {
    fnord()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("date"))
        .stdout(predicate::str::contains("cal"))
        .stdout(predicate::str::contains("pope"));
}

#[test]
fn test_date_help() {
    fnord()
        .args(["date", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--date"))
        .stdout(predicate::str::contains("--format"));
}

#[test]
fn test_date_format_flag() {
    fnord()
        .args(["date", "--date", "2025-01-01", "--format", "%A %B %d %Y"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Sweetmorn"))
        .stdout(predicate::str::contains("Chaos"))
        .stdout(predicate::str::contains("3191"));
}

#[test]
fn test_date_json_flag() {
    fnord()
        .args(["date", "--date", "2025-01-05", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"year\""))
        .stdout(predicate::str::contains("3191"));
}

#[test]
fn test_cal_all_flag() {
    fnord()
        .args(["cal", "--all"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Chaos"))
        .stdout(predicate::str::contains("Discord"))
        .stdout(predicate::str::contains("Aftermath"));
}

#[test]
fn test_cal_specific_season() {
    fnord()
        .args(["cal", "--season", "discord"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Discord"));
}

#[test]
fn test_stub_subcommands() {
    for sub in &["moon", "zodiac"] {
        fnord()
            .arg(sub)
            .assert()
            .success()
            .stdout(predicate::str::contains("coming soon"));
    }
}

#[test]
fn test_date_offset_positive() {
    fnord()
        .args(["date", "--date", "+0"])
        .assert()
        .success()
        .stdout(predicate::str::contains("YOLD"));
}

#[test]
fn test_date_today_keyword() {
    fnord()
        .args(["date", "--date", "today"])
        .assert()
        .success()
        .stdout(predicate::str::contains("YOLD"));
}

// ─── pope ──────────────────────────────────────────────────────────────────

#[test]
fn pope_exits_zero() {
    fnord()
        .arg("pope")
        .env("USER", "eris")
        .env("HOSTNAME", "archbox")
        .assert()
        .success()
        .stdout(predicate::str::contains("PAPAL DECLARATION"));
}

#[test]
fn pope_short_contains_username() {
    fnord()
        .args(["pope", "--short"])
        .env("USER", "eris")
        .env("HOSTNAME", "archbox")
        .assert()
        .success()
        .stdout(predicate::str::contains("eris:"));
}

#[test]
fn pope_bull_contains_decrees() {
    fnord()
        .args(["pope", "--bull"])
        .env("USER", "eris")
        .env("HOSTNAME", "archbox")
        .assert()
        .success()
        .stdout(predicate::str::contains("BULLA DISCORDIANA"))
        .stdout(predicate::str::contains("PAPAL DECREES"))
        .stdout(predicate::str::contains("Eris, Goddess of Chaos"));
}

#[test]
fn pope_json_is_valid_json() {
    let output = fnord()
        .args(["pope", "--json"])
        .env("USER", "eris")
        .env("HOSTNAME", "archbox")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let s = String::from_utf8(output).unwrap();
    let v: serde_json::Value = serde_json::from_str(&s).expect("invalid JSON");
    assert!(v.get("user").is_some());
    assert!(v.get("pope_title").is_some());
}

// ─── oracle ────────────────────────────────────────────────────────────────

#[test]
fn oracle_with_question_exits_zero() {
    fnord()
        .args(["oracle", "is a hotdog a sandwich"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ORACLE"));
}

#[test]
fn oracle_deterministic() {
    let out1 = fnord()
        .args(["oracle", "is a hotdog a sandwich"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let out2 = fnord()
        .args(["oracle", "is a hotdog a sandwich"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    assert_eq!(out1, out2, "oracle should be deterministic");
}

#[test]
fn oracle_json_is_valid_json() {
    let output = fnord()
        .args(["oracle", "what is a fnord", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let s = String::from_utf8(output).unwrap();
    let v: serde_json::Value = serde_json::from_str(&s).expect("invalid JSON");
    assert!(v.get("question").is_some());
    assert!(v.get("answer").is_some());
    assert!(v.get("seed").is_some());
}

// ─── fortune ───────────────────────────────────────────────────────────────

#[test]
fn fortune_exits_zero() {
    fnord()
        .arg("fortune")
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn fortune_count_returns_n() {
    let output = fnord()
        .args(["fortune", "--count", "3"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let s = String::from_utf8(output).unwrap();
    // 3 fortunes separated by "%\n" means 2 separators.
    let separators = s.lines().filter(|l| *l == "%").count();
    assert_eq!(separators, 2, "expected 2 separators for 3 fortunes, got {separators}: {s}");
}

#[test]
fn fortune_json_is_valid_json() {
    let output = fnord()
        .args(["fortune", "--json", "--count", "2"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let s = String::from_utf8(output).unwrap();
    let v: serde_json::Value = serde_json::from_str(&s).expect("invalid JSON");
    let arr = v.as_array().expect("expected array");
    assert_eq!(arr.len(), 2);
    assert!(arr[0].get("text").is_some());
    assert!(arr[0].get("tags").is_some());
    assert!(arr[0].get("source").is_some());
}

// ─── koan ──────────────────────────────────────────────────────────────────

#[test]
fn koan_exits_zero() {
    fnord()
        .arg("koan")
        .assert()
        .success()
        .stdout(predicate::str::contains("KOAN"));
}

#[test]
fn koan_count_returns_n() {
    let output = fnord()
        .args(["koan", "--count", "3"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let s = String::from_utf8(output).unwrap();
    let count = s.matches("KOAN").count();
    assert_eq!(count, 3, "expected 3 KOAN headings, got {count}: {s}");
}

#[test]
fn koan_seed_is_reproducible() {
    let out1 = fnord()
        .args(["koan", "--seed", "kallisti"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let out2 = fnord()
        .args(["koan", "--seed", "kallisti"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    assert_eq!(out1, out2, "same seed should produce same koan");
}

#[test]
fn koan_json_is_valid_json() {
    let output = fnord()
        .args(["koan", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let s = String::from_utf8(output).unwrap();
    let v: serde_json::Value = serde_json::from_str(&s).expect("invalid JSON");
    let arr = v.as_array().expect("expected array");
    assert_eq!(arr.len(), 1);
    assert!(arr[0].get("setup").is_some());
    assert!(arr[0].get("question").is_some());
    assert!(arr[0].get("response").is_some());
}
