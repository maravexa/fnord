use fn0rd_lib::subcommands::erisian::compute_diff;

#[test]
fn repeated_lines_keep_existing_dispute_boundaries() {
    // Moving the matching repeated line changes the reported disputes even when
    // the aggregate insert/delete counts are identical.
    let summary = compute_diff("b\na\na\n", "c\na\nc\n");
    assert_eq!(
        (summary.agreements, summary.order_lines, summary.chaos_lines),
        (1, 2, 2)
    );
    let disputes: Vec<_> = summary
        .disputes
        .iter()
        .map(|d| (d.line_number, d.order_lines.clone(), d.chaos_lines.clone()))
        .collect();
    assert_eq!(
        disputes,
        vec![
            (1, vec!["b".to_string()], vec!["c".to_string()]),
            (3, vec!["a".to_string()], vec!["c".to_string()])
        ]
    );
}

#[test]
fn repeated_lines_keep_zero_context_rendering() {
    let temp = tempfile::tempdir().unwrap();
    let a = temp.path().join("order.txt");
    let b = temp.path().join("chaos.txt");
    std::fs::write(&a, "b\na\na\n").unwrap();
    std::fs::write(&b, "c\na\nc\n").unwrap();
    let output = std::process::Command::new(assert_cmd::cargo::cargo_bin!("fnord"))
        .env_clear()
        .env("HOME", temp.path())
        .env("XDG_CONFIG_HOME", temp.path().join("config"))
        .args([
            "erisian",
            a.to_str().unwrap(),
            b.to_str().unwrap(),
            "--context",
            "0",
            "--no-color",
            "--no-unicode",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8(output.stdout)
        .unwrap()
        .contains("  [ORDER] b\n  [CHAOS] c\n\n  [ORDER] a\n  [CHAOS] c\n"));
}
