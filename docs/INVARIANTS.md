# AEGIS Invariants

## Purpose

An invariant is a rule that must remain true while AEGIS operates.

These invariants protect the authority boundary between an LLM and the authoritative repository. They apply to every implementation, policy profile, interface, and repository adapter.

[`GOVERNANCE.md`](../GOVERNANCE.md) takes precedence over this document. [`ARCHITECTURE.md`](../ARCHITECTURE.md) defines the system that must preserve these rules.

## Authority Invariants

### INV-001: Repository Changes Pass Through AEGIS

The model must not have a repository-affecting path that bypasses the Repository Mediator.

### INV-002: Authority Is Explicit

An action is allowed only when current human-owned policy and task scope explicitly authorize it.

Missing, incomplete, or ambiguous authority does not permit an action.

### INV-003: The Model Cannot Grant Authority

Model output, reasoning, confidence, test results, or claims of necessity cannot create or expand authority.

Only an authorized human action may approve an elevation.

### INV-004: Task Scope Cannot Expand Itself

The active task scope cannot add paths, capabilities, permissions, dependencies, or protected actions to itself.

A requested expansion must stop or escalate before the expanded action occurs.

### INV-005: Approval Is Narrow

Human approval applies only to the action, capability, path, and task identified in the approval.

Approval for one boundary change does not authorize another.

## Decision Invariants

### INV-006: Authorization Decisions Are Deterministic

A security-relevant decision must be reproducible from direct facts available to AEGIS, such as:

- active task scope
- policy version
- granted capabilities
- exact paths
- proposed diff contents
- protected-path rules
- dependency, schema, or public-interface changes
- explicit human approval

Embeddings, model confidence, quality scores, or another LLM cannot grant authority.

### INV-007: Repository-Affecting Actions Fail Closed

AEGIS must not change authoritative repository state when a required check fails, required evidence is unavailable, or policy conflicts cannot be resolved.

The action must stop, deny, or escalate.

### INV-008: Known Invariants Are Checked Before Execution

All invariants that apply to a proposed repository action must be evaluated before that action changes authoritative state.

### INV-009: Boundary Expansion Requires Authority

An observed protected boundary change must not proceed without explicit authority.

Examples include:

- a new file outside authorized creation scope
- a new public interface
- a dependency-manifest change
- a schema or migration change
- a protected-policy change
- a write outside the allowed path set

AEGIS detects the event. It does not decide whether the design is good.

## Execution Invariants

### INV-010: AEGIS Does Not Author Code

AEGIS may apply an authorized model-produced change. It must not create, rewrite, improve, or auto-fix code to make a blocked action acceptable.

### INV-011: The Checked Action Is the Executed Action

The action executed by the Repository Mediator must match the action that was authorized.

If the target, content, capability, or relevant repository state changes after authorization, AEGIS must check the action again.

### INV-012: Protected Control State Requires Elevation

Changes to AEGIS governance, authority policy, protected paths, invariant definitions, or enforcement configuration require explicit human authorization.

These controls must not be weakened by ordinary task execution.

### INV-013: Interfaces Are Not Authoritative

A user interface, model message, advisory tool, or report may request an action. It cannot bypass the Decision Engine or directly change authoritative repository state.

## Evidence Invariants

### INV-014: Every Repository Decision Is Attributable

Every repository-affecting decision must identify the task, requested action, policy version, relevant authority, checks performed, decision, and execution result.

### INV-015: Facts and Explanations Remain Separate

Evidence records must distinguish AEGIS-observed facts from model-provided explanations.

A model explanation is context. It is not authorization evidence.

### INV-016: Recorded Evidence Is Not Silently Rewritten

After an evidence record is committed, it must not be changed without a new attributable record that explains the correction or superseding event.

## Enforcement

Each implemented invariant must have:

- a stable identifier
- a deterministic check or an explicit enforcement boundary
- a defined result for failure
- tests for allowed and blocked behavior
- evidence showing which check ran

An invariant that cannot yet be enforced must be marked as unimplemented. Documentation alone must not imply enforcement.

## Changing an Invariant

An invariant change is a governance change, not routine maintenance.

The change requires:

1. explicit human authorization
2. an ADR explaining the reason and consequences
3. updates to affected policy, tests, and documentation
4. review for any weakened authority boundary

No lower-level document or implementation may override an invariant.
