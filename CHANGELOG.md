# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] — 0.1.2

### Changed

- Replaced `reqwest` with `ureq` in the `omens` subcommand. Removes the
  `tokio`, `hyper`, `rustls-platform-verifier`, and `aws-lc-rs` transitive
  dependency chain (~40 crates). Fixes AUR, Nix, and other downstream
  source builds that failed to link `aws-lc-sys` in hardened environments.
  The `omens` subcommand behavior is unchanged from a user perspective.
