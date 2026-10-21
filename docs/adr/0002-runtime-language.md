# ADR 0002: Runtime Language

## Status

Accepted. The human explicitly approved ADR 0002 for the Rust runtime on
2026-10-21 and authorized the first implementation.

## Date

2026-10-21

## Decision Owners

Repository owner `progentic` (Proto). Approval source: "I approve ADR-0002 for
Rust Runtime, proceed with implementation." The approved proposal is commit
`c55271102e55656f85909aa104c9bcf82ee7950f`, tree
`62c25f180b11907ce1d42a1b4d19946f38b109b5`.

## Phase and Version

Phase 2 (`v0.1.0`) onward. No roadmap outcome or phase exit gate changes.

## Affected Invariants

`INV-006`, `INV-007`, `INV-011`, `INV-012`.

No invariant wording or enforcement requirement changes.

## Context

ADR 0001 selects an Ubuntu 24.04 LTS deployment with separate executor,
mediator, and control-state identities. It requires a separate runtime-language
decision. The existing Python foundation validator is not a runtime selection.

## Decision

Use Rust for the authority contracts and subsequent trusted runtime.
Keep Python for existing read-only foundation tooling.

The first implementation is a small dependency-free library of authority
contract types, without a daemon, repository writes, executor isolation, or
claims that the Phase 2 exit gate is satisfied. Policy serialization and
capability semantics require their own recorded contract decisions.

Pin the Rust toolchain and commit the lockfile from the first runtime commit.
Approve third-party dependencies separately before adding them. Builds must
follow ADR 0001's controlled-build and provenance requirements.

## Alternatives Considered

- Python: reuses the foundation toolchain, but would make runtime contracts
  dynamically typed unless additional validation is implemented.
- Go: a compiled alternative; selecting it would require a different toolchain
  and library design. No existing runtime implementation constrains the choice.

## Consequences

- Rust types can represent distinct authority inputs without an execution API.
- Maintainers need a Rust toolchain in addition to the Python foundation tools.
- Language choice alone does not enforce authority or operating-system isolation.
- The first library represents unvalidated inputs; its types do not grant authority.

## Validation Evidence

The implementation commit must record the pinned toolchain and source revision,
pass formatting and compilation, and pass focused tests for implemented contract
behavior. Linux CI must build that exact commit on the supported platform.
These checks do not substitute for the later real-executor bypass evidence.

## Supersedes

None

## Superseded By

None
