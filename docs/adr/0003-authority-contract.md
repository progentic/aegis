# ADR 0003: Initial Authority Contract

## Status

Proposed. No implementation authority is granted by this record.

## Date

2026-10-21

## Decision Owners

Repository owner `progentic` (Proto). Explicit approval is pending.

## Phase and Version

Phase 2 (`v0.1.0`). Phase 3 decision outcomes and Phase 4 repository execution
remain outside this task. No roadmap outcome or phase exit gate changes.

## Affected Invariants

`INV-002`, `INV-003`, `INV-004`, `INV-005`, `INV-006`, `INV-007`, `INV-012`.
No invariant wording changes or runtime enforcement claims.

## Context

ADR 0002 approved Rust and a dependency-free library of unvalidated input types.
It requires separate decisions before policy serialization and capability
semantics are implemented. The next Phase 2 task is that contract, not the
Phase 3 decision engine.

## Decision

Propose the following initial contract:

- Use versioned JSON for human-owned policy and human-approved task scope.
  Require all authority fields explicitly; empty lists represent no grants.
  Reject unknown fields, duplicate object keys, unsupported schema versions,
  malformed values, and missing or blank identity/reference fields.
- Policy identifies the repository, owner, version, immutable policy reference,
  path ceiling, capability ceiling, and required invariant identifiers. Task
  scope identifies the same repository and policy, task ID, approval reference,
  requested paths, requested capabilities, and applicable invariant identifiers.
  Bind approval to the exact scope revision, not just its task ID.
- Treat owner and approval strings as declarations, never authentication.
  Trusted control-state loading must establish their provenance independently
  of model messages. Keep pure contract checks separate from that loader;
  successful parsing or validation must not issue an executable permit.
- Initially support exact repository-relative POSIX file paths only, with
  case-sensitive comparison. Reject absolute paths, empty components, dot or
  parent components, backslashes, NUL, and wildcard expressions rather than
  guessing their meaning. These lexical checks do not prove filesystem
  containment; symlinks and execution-time state belong to Phase 4.
- Require task requests to fit the policy ceiling. Read, write, and create stay
  distinct; no one operation implies another. A protected-action capability
  names both its category and exact target and must be present in both policy
  and task scope. Ordinary write authority cannot override protected actions.
- Reject requested scope or capabilities outside policy rather than silently
  widening or truncating them. Preserve required policy invariants and reject
  unknown invariant identifiers. Canonicalize set order for deterministic
  comparison; reject duplicate entries rather than silently discarding them.
- Use `serde` and `serde_json` for JSON parsing instead of writing a parser.
  Approval of this ADR would authorize these dependencies, pinned with their
  transitive inputs in `Cargo.lock`. Duplicate-key rejection must be explicit;
  parsing into a map that silently replaces an earlier key is insufficient.

## Alternatives Considered

- A custom JSON parser keeps third-party dependencies absent but adds parsing
  code to the trusted path. Not recommended.
- YAML and permissive globs add syntax and matching choices before the initial
  contract works. Defer them rather than interpreting them loosely.

## Consequences

Approval would authorize the strict parser and pure contract validation only.
No repository writes, decision engine, elevation workflow, permit lifecycle,
or executor isolation would be implemented under this decision.

Phase 2 remains incomplete until its exit evidence is recorded. Runtime use
still requires trusted provenance and the later enforcement boundaries.

## Validation Evidence

Implementation must provide focused fixtures for valid input, missing and
unknown fields, duplicate keys, policy/task mismatch, unsupported paths,
out-of-policy requests, and unknown or omitted required invariants. Repeated
inputs must yield the same result without modifying inputs or repository state.
The exact implementation commit must pass Linux CI and workflow lint.

## Supersedes

None

## Superseded By

None
