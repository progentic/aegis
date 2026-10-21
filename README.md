# AEGIS

**Status: Foundation stage.** AEGIS does not yet provide a supported runtime or out-of-the-box installation. Runtime enforcement and the `v1.0.0` production gates have not been validated. See the [roadmap](docs/ROADMAP.md) for planned milestones.

AEGIS exists because an LLM can produce code that is syntactically valid, compiles, and passes tests while still being outside the requested task, inconsistent with repository invariants, or based on an invented implementation direction. AEGIS reduces that risk by controlling what the model is permitted to do before repository state is changed.

AEGIS does not decide how software should be built. It enforces authority that humans have already defined.

## Rust authority types (maintainers)

The dependency-free `aegis-authority` library represents unvalidated task-scope,
policy-reference, capability, and protected-action inputs. These public data
types do not authenticate the human, validate paths or policy, grant permission,
or execute actions. All runtime invariants remain unimplemented. Phase 2 is not
complete. See [ADR 0002](docs/adr/0002-runtime-language.md).

From the repository root, with Rust installed through `rustup`, run:

```sh
cargo fmt --all -- --check
cargo clippy --locked --offline --all-targets -- -D warnings
cargo test --locked --offline
```

`rustup` installs the pinned toolchain and components if absent, requiring network
access. Cargo then checks the library without fetching third-party dependencies.
These commands generate local build output in `target/`; they do not mediate
repository actions.
Set `CARGO_TARGET_DIR` to an external build directory to keep the checkout free
of generated files. A toolchain or check failure is a build failure; there is no
runtime fallback or installation workflow yet.
