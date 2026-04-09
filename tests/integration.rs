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
fn test_pope_stub() {
    fnord()
        .arg("pope")
        .assert()
        .success()
        .stdout(predicate::str::contains("coming soon"));
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
    for sub in &["moon", "fortune", "pope", "oracle", "koan", "zodiac"] {
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
