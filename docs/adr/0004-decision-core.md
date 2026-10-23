# ADR 0004: Deterministic Decision Core

## Status

Proposed. The human accepted Phase 2 and instructed "start ADR-0004" on
2026-10-22. This authorizes the proposal, not its acceptance or implementation.

## Date

2026-10-22

## Decision Owners

Repository owner `progentic` (Proto), consistent with the accepted ADRs.

## Phase and Version

Phase 3, Decision Core (`v0.2.0`). The
[Phase 2 acceptance record](../acceptance/PHASE-2.md) identifies the accepted
contract implementation. This proposal leaves the roadmap sequence and exit
gates unchanged. Repository execution and Linux isolation remain Phase 4 work.

## Affected Invariants

`INV-002`, `INV-003`, `INV-004`, `INV-005`, `INV-006`, `INV-007`, `INV-008`,
`INV-009`, `INV-010`, `INV-011`, `INV-013`, `INV-014`, and `INV-015`.

No invariant wording changes. The proposed core compares authority and supplied
check results; it does not implement the operating-system or evidence-store
enforcement of those invariants.

## Context

The current Rust library validates declared task scope against independently
trusted policy and approved scope. It produces an immutable `ValidatedContract`
containing the complete canonical policy and task. It cannot yet decide whether
a particular proposed repository action fits that authority.

A scope comparison alone is insufficient. A request must name an exact
operation and target, carry the proposed bytes when it changes a file, and be
associated with complete check results for the same action and authority.
Missing checks must not become an empty violation list or an implicit pass.

The four outcomes and their authority boundaries are governed by
[Escalation Governance](../governance/ESCALATION_GOVERNANCE.md). The decision
pipeline must preserve those meanings and remain independent of filesystem,
network, model reasoning, and execution machinery.

## Decision

The following is proposed for human review.

### Pure Rust Interface

Add a pure evaluator to the existing library:

```rust
decide(contract: &ValidatedContract, request: &ActionRequest, checks: &CheckReport) -> Decision
```

The core validates its request and check-report inputs before applying the
ordered decision rules below. A failed Phase 2 contract validation produces no
contract to pass to this interface. Missing policy must never be replaced by an
empty or permissive contract.

Reuse the approved parser dependencies and existing limits and lexical checks.
This proposal adds no dependency, policy field, authority source, external wire
format, daemon, or execution API. Source loading and authentication retain the
trusted-caller preconditions of [ADR 0003](0003-authority-contract.md).

### Action Request

An `ActionRequest` contains these bindings:

| Field | Meaning |
| :--- | :--- |
| Repository identifier | Exact match with the contract repository |
| Task identifier | Exact match with the contract task |
| Full policy reference | Owner, version, and immutable reference from the contract |
| Approval reference | Exact match with the approved task |
| Action | One of the operation shapes below |

Use a closed action enum:

- `Read` carries the exact target path.
- `Write` carries the exact target path and required proposed bytes.
- `Create` carries the exact target path and required proposed bytes.

Apply the existing identity and exact-path limits. Cap proposed bytes at
262,144 bytes per action, using a named action-size constant. Empty content is
valid file content; missing content is not a `Write` or `Create` shape.

Read, write, and create remain distinct. A request needs its operation-specific
path bound and explicit capability. A protected-action capability is additional
authority for a reported boundary event; it is not an executable operation or a
substitute for ordinary file authority. Process, network, secret access,
arbitrary shell commands, and caller-created permits have no request variant.

Do not accept a caller-provided content hash as proof of checked content. Retain
the exact proposed bytes in the immutable action binding. Phase 4 must check the
executed operation, target, bytes, and relevant filesystem state again.

### Check Report

A `CheckReport` is data supplied by trusted checking code. It is not a model
claim, authority grant, authenticated type, or proof that a checker ran. A
future gateway must construct it independently of the model-facing request.
The pure core cannot establish that provenance itself.

Bind each report to the complete contract snapshot and exact action request,
including proposed bytes. A report for another policy body, task, operation,
target, or content cannot be reused. Reference strings alone are insufficient.

The report contains:

- An explicit control-state assessment: current, conflicting, or unavailable.
  Freshness is a supplied fact; the pure core does not read a clock or store.
- Exactly one result for every invariant required by the contract task. A
  result is satisfied, violated, explicitly prohibited, or unavailable.
- An explicit boundary assessment: complete with observed events, or
  unavailable. Each event identifies the existing protected-action category
  and exact target that require authority.

An explicitly prohibited result must identify a human-owned rule that forbids
the action without elevation. A rule requiring approval produces a required
protected action rather than inventing a non-elevatable prohibition. This does
not add a deny-list field to the existing policy schema.

Report validation rejects duplicate, unknown, contradictory, or mismatched
results. Missing required results remain unavailable. A complete empty boundary
assessment means the trusted checker ran every applicable configured check and
found no events; it must never be synthesized from missing input.

The core consumes check results. Actual invariant enforcement, repository
inspection, boundary detection, trusted loading, and current-state verification
remain with their respective roadmap components. An unavailable component
cannot be represented as a successful empty stub.

### Decision Ordering and Reason Codes

Apply the first matching rule in this order. Each result has one primary reason
code; a known prohibition uses `DENY`, and a request needing human authority uses
`ESCALATE`. Blocked outcomes authorize no execution.

| Order | Observed condition | Outcome | Primary reason code |
| :--- | :--- | :--- | :--- |
| 1 | Malformed request, invalid target, or oversized action | `STOP` | `REQUEST_INVALID` |
| 2 | Request repository, task, policy, or approval binding differs from the contract | `STOP` | `AUTHORITY_BINDING_MISMATCH` |
| 3 | Check report is contradictory, has duplicate or unknown results, or belongs to another contract/action | `ESCALATE` | `CHECK_REPORT_INVALID` |
| 4 | Required control state is conflicting or unavailable | `ESCALATE` | `CONTROL_STATE_UNAVAILABLE` |
| 5 | A valid check establishes an explicit prohibition without elevation | `DENY` | `EXPLICITLY_PROHIBITED` |
| 6 | The ordinary operation lacks its path bound or capability in the policy ceiling | `STOP` | `POLICY_AUTHORITY_MISSING` |
| 7 | The operation is within the policy ceiling but lacks its path bound or capability in the task | `ESCALATE` | `TASK_SCOPE_EXCEEDED` |
| 8 | An applicable invariant is violated | `STOP` | `INVARIANT_VIOLATION` |
| 9 | An observed boundary event lacks the exact protected grant in both policy and task | `ESCALATE` | `PROTECTED_ACTION_REQUIRED` |
| 10 | Any required invariant result or complete boundary assessment is unavailable | `ESCALATE` | `CHECK_EVIDENCE_UNAVAILABLE` |
| 11 | All applicable authority, invariant, boundary, and completeness checks pass | `ALLOW` | `ACTION_AUTHORIZED` |

Rows 6 and 7 distinguish absence of policy authority from a request to extend
task scope within the human-owned ceiling. The ceiling alone never authorizes
execution. A protected grant is matched by both category and target.

Canonicalize sets of supplied facts before comparison. Reordering invariant
results or boundary events must not change the result. Where a reason identifies
a particular failed check, choose it in canonical order. No model explanation,
confidence value, passing test, or code-quality judgment affects the outcome.

### Decision Result

`Decision` exposes its outcome and typed reason code through shared accessors.
It retains the immutable authority and action bindings needed to explain the
comparison. Keep its fields private and do not deserialize a `Decision` from
caller JSON. Debug and Display output must not include proposed bytes, request
strings, or model explanations.

A decision is a comparison result, not a permit. `ALLOW` means the supplied
action and trusted facts satisfied this pure evaluation. It does not establish
human authentication, current filesystem containment, permit freshness, or
authority for a different action. Later execution must use the mediated path
required by [ADR 0001](0001-deployment-profile-and-threat-model.md).

The result can supply inputs to later evidence recording. Phase 3 does not
persist records or claim that `INV-014` is enforced by returning a value.

## Alternatives Considered

- **Execute immediately after a boolean check.** Rejected: this would combine
  policy with I/O and bypass the later repository and isolation requirements.
- **Use optional check lists or a `sufficient_evidence` boolean.** Rejected:
  missing results could be mistaken for successful enforcement.
- **Accept serialized decisions or model-authored check reports as authority.**
  Rejected: the requester cannot supply its own authorization evidence.
- **Infer boundary or design quality with another model.** Rejected: governance
  requires direct facts and excludes probabilistic authorization.

## Consequences

The core becomes testable as a deterministic mapping from authority, an action,
and explicit check results to an outcome. Required missing facts block progress
without introducing an I/O or reasoning fallback.

Its trust assumptions remain visible. Fixtures can test the decision logic but
cannot prove that runtime checks, caller identity, filesystem isolation, evidence
persistence, or permit lifecycle controls exist. The Phase 4 through Phase 8
gates retain those responsibilities.

Accepting this ADR would authorize the described pure Rust types, evaluator,
reason codes, tests, and maintainer reference. It would not authorize repository
writes, a daemon, a new dependency, schema expansion, or production admission.

## Validation Evidence

For the proposal: verify links, the ADR index, Phase 2 source and CI identity,
and consistency with the four governance documents. No Phase 3 implementation
or test result is claimed by this proposed record.

If accepted, implementation must provide table-driven tests for every decision
row and every competing-condition precedence pair that could change the outcome.
Include:

- Separate read, write, and create authority, including path-only and
  capability-only grants that must not suffice.
- Repository, task, full-policy, approval, target, operation, and byte changes
  that invalidate a reused request or report.
- Explicit prohibitions, invariant violations, and protected category/target
  mismatches, including policy-only and task-only protected grants.
- Missing, unavailable, duplicate, unknown, contradictory, and reordered check
  results; no default pass for missing evidence.
- Exact request size limits, redacted diagnostics, immutable result access,
  and rejection of caller-deserialized decisions.
- Repeated-input determinism and preservation of input values.

The exact implementation commit must pass formatting, Clippy, build, the full
Rust and foundation suites, and Ubuntu 24.04 CI with workflow lint. Record the
implemented limits and remaining runtime requirements in the maintainer reference.

## Supersedes

None

## Superseded By

None
