# Contributing to fnord

All Hail Eris. Pull requests welcome.

---

## Prerequisites

- Rust **1.88** or later (the pinned toolchain in `rust-toolchain.toml` is picked up automatically by `rustup`)
- The `rustfmt` and `clippy` components — `rustup` installs them from the toolchain file, or manually:
  ```bash
  rustup component add rustfmt clippy
  ```
- [`cargo-deny`](https://github.com/EmbarkStudios/cargo-deny) for the security/license check:
  ```bash
  cargo install cargo-deny
  ```

---

## Building and testing

```bash
cargo build                                        # debug build
cargo build --release                              # release build
cargo test --workspace --all-targets               # full test suite
cargo test --workspace --doc                       # doctests
```

To generate man pages and shell completions:

```bash
cargo build --features generate-assets
cargo run --bin generate-completions --features generate-assets
# outputs: man/  completions/
```

---

## Before opening a PR

CI enforces all of the following. Run them locally first to avoid round-trips:

```bash
cargo fmt --all                                                       # format
cargo fmt --all -- --check                                            # check only (what CI does)
cargo clippy --workspace --all-targets --all-features -- -D warnings  # lint
cargo test --workspace --all-targets                                  # tests
cargo doc --workspace --all-features --no-deps --document-private-items  # docs
cargo deny check                                                      # advisories, licenses, bans
```

`RUSTFLAGS="-D warnings"` is set in CI, so all compiler warnings are treated as errors.

---

## Commit style

This project uses [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(subcommand): add fnord to the list of things
fix(date): off-by-one on St. Tib's Day
refactor(omens): swap dependency
docs: update README
ci: add arm64 coverage
```

Keep the subject line under 72 characters. The body is optional but appreciated for non-obvious changes.

---

## Pull requests

- Target the `main` branch.
- One logical change per PR.
- If you're adding a new subcommand, include integration tests under `tests/`.
- Update `CHANGELOG.md` under `[Unreleased]` following the [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) format.

---

## MSRV policy

The minimum supported Rust version is **1.88**. New dependencies must build on 1.88. If you need a feature from a newer compiler, open an issue first to discuss bumping the MSRV.

---

## Dependency guidelines

- Prefer crates already in the tree before adding new ones.
- Avoid crates that require native system libraries unless strictly necessary (the `aws-lc-sys` removal in 0.1.2 is a good example of why).
- `cargo deny check` must pass — check `deny.toml` for the license allow-list.

---

## Releases

Releases are cut by pushing a `vMAJOR.MINOR.PATCH` tag to `main`. The release workflow builds `.deb`, `.rpm`, and `.tar.gz` artifacts for `amd64` and `arm64` automatically. Ordinary contributors do not need to worry about this.

---

## License

By contributing you agree that your changes will be released under the [WTFPL](LICENSE). This means you can do what the fuck you want with them, and so can everyone else.
