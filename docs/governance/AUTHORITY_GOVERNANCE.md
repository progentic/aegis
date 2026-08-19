# Authority Governance

## Purpose

This document defines how AEGIS decides what an LLM is authorized to do.

AEGIS governs authority. It does not govern reasoning quality or software design quality.

## Authority Is Explicit

Authority must be affirmatively granted by human-owned policy, task scope, or an approved elevation.

AEGIS must not infer authority from:

- silence
- incomplete configuration
- repository convention alone
- model explanation
- likely human intent
- previous unrelated approvals
- a passing test suite
- successful compilation

If an action is not explicitly authorized, it is unauthorized.

## Sources of Authority

AEGIS may recognize authority from the following sources only:

1. Human-owned repository policy.
2. Human-approved task scope.
3. A current capability or lease derived from that policy and scope.
4. A specific human-approved elevation.

An LLM cannot create, extend, reinterpret, or approve its own authority.

## Task Scope

Every execution must have a task scope.

At minimum, task scope should identify the repository actions the LLM may perform, including applicable read and write boundaries.

A task may also explicitly authorize protected actions such as:

- creating a new module
- changing a dependency manifest
- modifying a schema
- changing authorization policy
- introducing a service connection
- modifying a protected directory

If the task does not authorize such an action, AEGIS treats the action as outside scope.

## Filesystem Authority

Write authority is allowlisted.

A file is writable only when its path is covered by an active, explicit write authorization.

Examples:

```text
read:
  src/auth/**
  tests/auth/**

write:
  src/auth/token.rs
  tests/auth/token_test.rs

deny unless separately elevated:
  Cargo.toml
  migrations/**
  .github/**
  policy/**
  aegis/**
```

A path not covered by a write authorization is not writable.

## Capability Model

Capabilities should be:

- explicit
- narrow
- attributable
- revocable
- time-bounded when practical
- auditable

A capability grants authority to perform a specific class of action. It does not grant authority to reinterpret task intent.

## Repository Invariants

AEGIS may enforce machine-readable repository invariants that humans have already defined.

Examples:

- generated files cannot be edited directly
- UI code cannot write authoritative state
- schema changes require elevation
- dependency changes require elevation
- authentication or authorization changes require elevation
- protected policy directories require explicit authorization

AEGIS enforces these decisions. It does not invent them.

## Model Claims Are Not Authority

The model may explain why it believes an action is necessary.

That explanation is evidence for a human decision, not authorization.

Statements such as the following do not expand authority:

- "This dependency is required."
- "The architecture needs a new module."
- "The migration is safe."
- "The user probably intended this change."
- "Tests pass, so this is acceptable."

## Authority Precedence

When multiple policies apply, the more restrictive authority wins unless a specific human-approved elevation explicitly overrides it.

An elevation must identify what restriction is being overridden and the scope of the override.

## Prohibited Behavior

AEGIS must not:

- infer permission from absence of a deny rule
- allow an LLM to modify the policy governing its own execution unless explicitly authorized by a human
- allow an LLM to grant itself new capabilities
- convert model confidence into authority
- use test success as proof of authorization
- silently broaden task scope

## Decision Requirement

For every repository-affecting action, AEGIS must be able to identify the explicit authority that permits it.

If it cannot, the action is denied or escalated according to `ESCALATION_GOVERNANCE.md`.
