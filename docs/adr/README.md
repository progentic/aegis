# Architecture Decision Records

## Purpose

An Architecture Decision Record (ADR) preserves one durable project decision and the evidence available when that decision was made. ADRs make changes to system boundaries, authority sources, invariants, dependency direction, evidence contracts, and failure models reviewable without rewriting history.

## When an ADR Is Required

Create an ADR before implementing a decision that changes:

- a durable architecture or authority boundary
- an authority source or protected action
- an invariant or its enforcement boundary
- a dependency direction
- an evidence contract or retention rule
- a failure or recovery model
- a roadmap phase sequence or exit gate

Routine implementation details that follow an existing decision do not require an ADR.

## Location and Naming

This directory is the canonical ADR location. Use `NNNN-short-title.md`, where `NNNN` is the next unused four-digit number and `short-title` is a brief lowercase hyphenated name.

`0000-template.md` is reserved for the template and is not a decision. The first decision record is `0001-short-title.md`. Numbers are never reused, even when a record is rejected, deprecated, or superseded.

## Allowed Statuses

| Status | Meaning |
| :--- | :--- |
| Proposed | The decision is under review and is not approved. |
| Accepted | The decision is approved and controls its stated scope. |
| Rejected | The proposal was considered and not approved. |
| Superseded | A later ADR replaces this decision. |
| Deprecated | The decision still exists but should not guide new work. |

## Superseding a Decision

Do not rewrite an accepted ADR to replace its decision. Create a new ADR, identify the earlier record in `Supersedes`, and update the earlier record's status to `Superseded` with the new identifier in `Superseded by`.

This preserves the decision history required by `INV-016`.

## Required References

Every ADR must identify:

- affected invariant identifiers, or `None`
- the affected roadmap version or versions
- the affected phase or phases
- validation evidence that can be checked directly

An ADR may refine implementation order through the phase map. It cannot silently weaken governance, override an invariant, expand task scope, or change the human-owned roadmap.

## Decision Index

No decision ADRs exist yet. Phase 1 establishes this process without inventing an architectural decision.

| Identifier | Record | Status |
| :--- | :--- | :--- |
| `0000` | [`0000-template.md`](0000-template.md) | Template, not a decision |
