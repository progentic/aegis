# AEGIS

**Status: Early development; authority-contract library only.** AEGIS does not yet provide a supported runtime or out-of-the-box installation. Runtime enforcement and the `v1.0.0` production gates have not been validated. See the [roadmap](docs/ROADMAP.md) for planned milestones.

AEGIS exists because an LLM can produce code that is syntactically valid, compiles, and passes tests while still being outside the requested task, inconsistent with repository invariants, or based on an invented implementation direction. AEGIS reduces that risk by controlling what the model is permitted to do before repository state is changed.

AEGIS does not decide how software should be built. It enforces authority that humans have already defined.

## Rust authority contract (maintainers)

The `aegis-authority` library parses strict JSON declarations and checks task
scope against independently trusted policy and human-approved scope. It does
not authenticate those sources, issue permits, or execute actions. Public input
types alone remain unvalidated. See the [contract reference](docs/AUTHORITY_CONTRACT.md)
and [ADR 0003](docs/adr/0003-authority-contract.md) for the trust boundary and limits.

From the repository root, with Rust installed through `rustup`, run:

```sh
cargo fetch --locked
cargo fmt --all -- --check
cargo clippy --locked --offline --all-targets -- -D warnings
cargo test --locked --offline
```

`rustup` installs the pinned toolchain and components if absent, requiring network
access. `cargo fetch --locked` downloads the approved parser dependencies at
their committed versions and checksums. Subsequent checks run offline.
These commands generate local build output in `target/`; they do not mediate
repository actions.
Set `CARGO_TARGET_DIR` to an external build directory to keep the checkout free
of generated files. A toolchain or check failure is a build failure; there is no
runtime fallback or installation workflow yet.
