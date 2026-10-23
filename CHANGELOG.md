# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog, and this project adheres to Semantic Versioning.

## [0.0.0] - 2026-08-18

### Added

- Bounded authority inputs, immutable full-policy/task snapshots, redacted diagnostics, and security regression checks; secret, process, network, and executable-permit inputs remain unsupported, and runtime isolation remains unimplemented.
- Implemented the ADR 0003-approved strict JSON parser and pure policy/task/approval validator, with pinned parser dependencies, deterministic output, and rejection fixtures; runtime enforcement remains unimplemented.
- Added a checksum-pinned GitHub Actions Lint job that must pass before foundation and Rust checks run.
- Added the ADR 0002-approved, dependency-free Rust authority input types, pinned toolchain, lockfile, representation tests, and Linux build checks; no runtime enforcement is implemented.
- Established the initial AEGIS project foundation and Phase 1 exit evidence.
- Added the Architecture Decision Record process and template.
- Added deterministic, read-only validation for required Phase 1 files, bootstrap documentation routes, local Markdown references, the Keep a Changelog preamble, the initial `0.0.0` version, and prohibited `Unreleased` headings.
- Added focused standard-library tests for the foundation validator.
- Added exact-letter-case checking for local Markdown references and a Linux GitHub Actions workflow that runs the validator tests and foundation checks.
- Added a foundation-stage status notice to the README and a commit-bound Phase 1 acceptance record.
