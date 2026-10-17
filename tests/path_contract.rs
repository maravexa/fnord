#![cfg(target_os = "linux")]

use std::path::Path;
use std::process::{Command, Output};

fn run(home: &Path, cwd: &Path, xdg: Option<&Path>, args: &[&str]) -> Output {
    let mut command = Command::new(assert_cmd::cargo::cargo_bin!("fnord"));
    command
        .env_clear()
        .env("HOME", home)
        .current_dir(cwd)
        .args(args);
    if let Some(xdg) = xdg {
        command.env("XDG_CONFIG_HOME", xdg);
    }
    let output = command.output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

#[test]
fn config_and_personal_holydays_keep_existing_xdg_and_home_locations() {
    for mode in ["unset", "absolute", "relative", "empty"] {
        let temp = tempfile::tempdir().unwrap();
        let home = temp.path().join("home");
        let custom = temp.path().join("custom");
        let fallback = home.join(".config/eris");
        let selected = if mode == "absolute" {
            custom.join("eris")
        } else {
            fallback.clone()
        };
        std::fs::create_dir_all(&fallback).unwrap();
        std::fs::create_dir_all(custom.join("eris")).unwrap();
        let default_config = "[identity]\npope_title = 'HOME marker'\n";
        let custom_config = "[identity]\npope_title = 'XDG marker'\n";
        std::fs::write(fallback.join("fnord.toml"), default_config).unwrap();
        std::fs::write(custom.join("eris/fnord.toml"), custom_config).unwrap();
        let xdg = match mode {
            "absolute" => Some(custom.as_path()),
            "relative" => Some(Path::new("relative-config")),
            "empty" => Some(Path::new("")),
            _ => None,
        };
        let output = run(&home, temp.path(), xdg, &["pope", "--short"]);
        let marker = if mode == "absolute" {
            "XDG marker"
        } else {
            "HOME marker"
        };
        assert!(
            String::from_utf8_lossy(&output.stdout).contains(marker),
            "{mode}"
        );
        run(
            &home,
            temp.path(),
            xdg,
            &["holyday", "add", "chaos-42", "Location marker"],
        );
        let personal = selected.join("holydays/personal.toml");
        assert!(
            std::fs::read_to_string(personal)
                .unwrap()
                .contains("Location marker"),
            "{mode}"
        );
        assert_eq!(
            std::fs::read_to_string(fallback.join("fnord.toml")).unwrap(),
            default_config
        );
        assert_eq!(
            std::fs::read_to_string(custom.join("eris/fnord.toml")).unwrap(),
            custom_config
        );
        assert!(!temp.path().join("relative-config").exists());
        let unselected = if mode == "absolute" {
            fallback
        } else {
            custom.join("eris")
        };
        assert!(
            !unselected.join("holydays/personal.toml").exists(),
            "{mode}"
        );
    }
}

#[test]
fn default_grimoire_keeps_home_location_even_with_xdg_override() {
    let temp = tempfile::tempdir().unwrap();
    let home = temp.path().join("home");
    let custom = temp.path().join("custom");
    std::fs::create_dir_all(&home).unwrap();
    let expected = home.join(".config/eris/grimoire");
    std::fs::create_dir_all(expected.parent().unwrap()).unwrap();
    std::fs::write(&expected, "Existing grimoire marker\n").unwrap();
    run(
        &home,
        temp.path(),
        Some(&custom),
        &["log", "New grimoire marker"],
    );
    let content = std::fs::read_to_string(expected).unwrap();
    assert!(content.starts_with("Existing grimoire marker\n"));
    assert!(content.contains("New grimoire marker"));
    assert!(!custom.join("eris/grimoire").exists());
}

#[test]
fn explicit_tilde_log_path_preserves_home_expansion() {
    let temp = tempfile::tempdir().unwrap();
    let home = temp.path().join("home");
    std::fs::create_dir_all(&home).unwrap();
    run(
        &home,
        temp.path(),
        None,
        &[
            "log",
            "Explicit path marker",
            "--file",
            "~/journal/grimoire",
        ],
    );
    assert!(std::fs::read_to_string(home.join("journal/grimoire"))
        .unwrap()
        .contains("Explicit path marker"));
    assert!(!temp.path().join("~").exists());
}
