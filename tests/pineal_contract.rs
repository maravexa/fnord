#![cfg(target_os = "linux")]

#[test]
fn pineal_preserves_metric_units_and_json_shape() {
    let home = tempfile::tempdir().unwrap();
    let output = std::process::Command::new(assert_cmd::cargo::cargo_bin!("fnord"))
        .env_clear()
        .env("HOME", home.path())
        .env("XDG_CONFIG_HOME", home.path().join("config"))
        .args(["pineal", "--json", "--no-color"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!output.stdout.contains(&0x1b));
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let keys: Vec<_> = value
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(
        keys,
        [
            "consciousness",
            "cpu",
            "discordian_date",
            "host",
            "load",
            "memory",
            "os",
            "prophecy",
            "uptime_seconds",
            "user"
        ]
    );

    // Linux reports MemTotal in KiB; the public CLI contract reports bytes.
    let meminfo = std::fs::read_to_string("/proc/meminfo").unwrap();
    let total_kib: u64 = meminfo
        .lines()
        .find_map(|line| line.strip_prefix("MemTotal:"))
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let memory = &value["memory"];
    assert_eq!(memory["total_bytes"].as_u64(), Some(total_kib * 1024));
    let used = memory["used_bytes"].as_u64().unwrap();
    assert!(used <= total_kib * 1024);
    let expected_pct = 100.0 * used as f64 / (total_kib * 1024) as f64;
    assert!((memory["used_pct"].as_f64().unwrap() - expected_pct).abs() < 1e-9);
    let cores = value["cpu"]["core_count"].as_u64().unwrap();
    assert!(cores > 0);
    let usage = value["cpu"]["per_cpu_usage"].as_array().unwrap();
    assert_eq!(usage.len() as u64, cores);
    assert!(usage
        .iter()
        .all(|item| (0.0..=100.0).contains(&item.as_f64().unwrap())));
    assert!(value["uptime_seconds"].as_u64().unwrap() > 0);
    for period in ["one", "five", "fifteen"] {
        assert!(value["load"][period].as_f64().unwrap() >= 0.0);
    }
    assert!((0..=100).contains(&value["consciousness"]["level"].as_i64().unwrap()));
}
