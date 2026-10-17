#![cfg(feature = "generate-assets")]

// Exercise the same command builder as asset generation without relying on
// build-script filesystem side effects being rerun in a reused target directory.
include!("../build.rs");

#[test]
fn generated_manual_pages_preserve_cli_documentation() {
    for (name, expected) in [
        ("fnord.1", include_str!("manual_snapshots/fnord.1.json")),
        (
            "fnord-date.1",
            include_str!("manual_snapshots/fnord-date.1.json"),
        ),
        (
            "fnord-holyday.1",
            include_str!("manual_snapshots/fnord-holyday.1.json"),
        ),
    ] {
        let expected: Vec<String> = serde_json::from_str(expected).unwrap();
        let expected = expected.concat();
        let cmd = build_command();
        let command = match name {
            "fnord.1" => cmd,
            "fnord-date.1" => cmd
                .find_subcommand("date")
                .unwrap()
                .clone()
                .name("fnord-date"),
            "fnord-holyday.1" => cmd
                .find_subcommand("holyday")
                .unwrap()
                .clone()
                .name("fnord-holyday"),
            _ => unreachable!(),
        };
        let mut rendered = Vec::new();
        clap_mangen::Man::new(command)
            .render(&mut rendered)
            .unwrap();
        let actual = String::from_utf8(rendered).unwrap();
        assert_eq!(actual, expected, "generated manual changed: {name}");
    }
}
