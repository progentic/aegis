# Authority Contract — Maintainer Reference

## Status and Boundary

Implemented under [ADR 0003](adr/0003-authority-contract.md): strict JSON parsing
and pure policy/task/approval comparison. Rust and dependency versions are
pinned in the toolchain file, manifest, and lockfile.

This is not a running gateway. It performs no file access, authentication,
repository mutation, decision-outcome selection, or permit issuance. All
runtime invariant enforcement remains unimplemented.

## Inputs

Both document kinds require `schema_version: 1` and a nonblank `repository_id`.
All fields shown in the [policy fixture](../tests/fixtures/policy.json) and
[task fixture](../tests/fixtures/task.json) are required, including empty lists.
Unknown fields, duplicate keys, malformed types, and trailing JSON are rejected.
Objects cannot be replaced by positional arrays.

The policy document contains `policy`, `paths`, `capabilities`, and `invariants`.
The task document contains `scope`, which holds the existing task input fields.
Policy references require `owner`, `version`, and `reference`. These values
declare identity; they do not prove it.

Capabilities use externally tagged objects: `{"ReadPath":"src/a.rs"}`,
`{"WritePath":"src/a.rs"}`, `{"CreatePath":"tests/new.rs"}`, or
`{"ProtectedAction":{"action":"DependencyChange","target":"Cargo.toml"}}`.
Protected-action names are the `ProtectedAction` enum variants. Operations are
distinct. A protected grant never supplies an ordinary read/write/create grant.
Path lists are bounds, not implicit capabilities; later decisions must require
the explicit capability as well as its applicable path bound.

## API and Trust Preconditions

- `parse_policy(text)` parses and structurally validates a policy declaration.
- `parse_task(text)` parses and structurally validates a task declaration.
- `validate_contract(policy, requested, approved)` checks the requested task
  against the policy ceiling and the entire independently approved task.
- `ValidatedContract::task()` returns an immutable view of canonical task data.
  It is not an execution permit.

The trusted caller must obtain `policy` and `approved` from independently
verified, human-controlled state, never from the requesting model. This library
does not load or authenticate that state. Providing three invented documents
cannot establish human authority, even if their strings agree.

Approval is bound to all canonical document fields, not just task ID or approval
reference. Object layout and set order are not revision identity; changed
scope, policy reference, capabilities, or invariants require renewed approval.
The trusted loader must establish that policy references are immutable and
approvals are current. Its deployment boundary is not implemented here.

## Validation and Failure Behavior

Identity/reference strings must be nonblank. Paths must be exact, relative POSIX
file paths: no empty, dot, or parent components, backslashes, NUL, or wildcard
characters. Comparison is case-sensitive. No glob or substring matching occurs.
Lexical validation does not establish symlink containment or execution safety.

Each ordinary capability must fit its corresponding path list. Task path lists
and capabilities must fit the matching policy ceilings; requests are rejected,
not truncated. Protected grants must match both category and exact target.
Policy-required invariant IDs cannot be omitted. Only `INV-001` through
`INV-016` are recognized; recognizing an ID does not execute its invariant.

Duplicate set entries are rejected. Successful output is sorted deterministically.
Validation clones output data and does not mutate inputs. Failures return
`ContractError` without producing a `ValidatedContract`; there is no fallback.

## Evidence and Remaining Work

[Contract tests](../tests/contract_validation.rs) cover schema rejection,
invalid paths, provenance fields, policy ceilings, exact approval binding,
required invariants, determinism, and input preservation. The three original
representation tests remain; foundation tests also cover this document's links.

Run the [README checks](../README.md) from the repository root. Linux CI also
records the source commit/tree, toolchain versions, and library artifact checksum.

This supports the pure contract portion of `INV-002` through `INV-007`; it does
not establish their runtime enforcement. Phase 3 must implement reproducible
decision outcomes. Phase 4 must enforce trusted loading and repository mediation
on the approved deployment profile. Elevation, evidence, stale-state handling,
and production admission retain their later roadmap gates.
