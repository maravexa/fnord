#[cfg(feature = "generate-assets")]
#[test]
fn generated_manual_pages_preserve_cli_documentation() {
    for (name, expected) in [
        ("fnord.1", include_str!("fixtures/man/fnord.1.json")),
        (
            "fnord-date.1",
            include_str!("fixtures/man/fnord-date.1.json"),
        ),
        (
            "fnord-holyday.1",
            include_str!("fixtures/man/fnord-holyday.1.json"),
        ),
    ] {
        let expected: Vec<String> = serde_json::from_str(expected).unwrap();
        let expected = expected.concat();
        let actual = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("man")
                .join(name),
        )
        .unwrap();
        assert_eq!(actual, expected, "generated manual changed: {name}");
    }
}
