# fnord repository instructions

A Discordian calendar and chaos CLI. Preserve its tone, calendar semantics and predictable errors and machine-readable output. `Cargo.toml` declares package `fn0rd` 0.1.2, edition 2021 and minimum Rust 1.95; `rust-toolchain.toml` pins 1.95.0 with rustfmt and Clippy. The CLI binary is `fnord`; the library is `fn0rd_lib`.

## Source of truth

Read `src/cli.rs`, the dispatcher in `src/main.rs` and the relevant module under `src/subcommands/` together. Commands now dispatch to real implementations, including pope, fortune, moon, zodiac, omens, log and pineal; the earlier stub-only description is obsolete. A dispatcher does not establish behavioral acceptance. Follow the corresponding unit and integration contracts under `tests/`.

- `src/date/` owns date conversion and St Tib's Day; `src/holydays/` owns definitions, registries and types. Preserve leap-year rules, weekday/season order, YOLD arithmetic, holyday keys and format tokens.
- `src/moon/`, `src/zodiac/` and `src/wake/` hold specialized calculations and presentation helpers.
- `src/config/schema.rs` owns defaults and `src/config/mod.rs` owns layered loading.
- `src/error.rs` owns typed errors. `tests/integration.rs` and the dependency, error, path, manual, Erisian and pineal contract tests cover different public boundaries.
- `build.rs` generates manual assets; `src/bin/generate-completions.rs` requires `generate-assets`. Inspect generated-output paths before running these commands.

Configuration precedence, later overriding earlier: schema defaults, `/etc/eris/fnord.toml`, the user config directory's `eris/fnord.toml`, `FNORD_CONFIG_` field overrides (underscore splitting), the file named by `FNORD_CONFIG`, then `--config`. Loading creates the user `eris` config directory. Use disposable XDG config/data/cache paths and review inherited FNORD configuration before tests or smoke runs.

Do not treat every CLI command as read-only: holyday add/remove persists personal records, log commands write a grimoire and may launch an editor, and omens can request weather from wttr.in. Inspect the selected command before execution and use isolated files and controlled endpoints. Holyday supports personal entries and its action-specific JSON options; the global JSON argument is not automatically honored by every action. Test the precise public option rather than promising blanket support.

## Validation and contribution

`CONTRIBUTING.md`, the manifest and `deny.toml` own dependency, change-log and contribution details. Preserve unrelated manifest edits in other checkouts. Run applicable checks locally; do not trigger private hosted CI or release tags as a side effect of validation.

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets
cargo test --workspace --doc
cargo doc --workspace --all-features --no-deps --document-private-items
cargo deny check
```

These commands require available tools/dependencies and may build or download them. For documentation-only reconciliation, inspect source and manifests without installing tools or running builds. Report checks actually run and their limits. Ordinary code validation does not authorize publishing packages, pushing a release tag, launching a user's editor or writing their records.
