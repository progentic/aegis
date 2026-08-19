# AEGIS Phase Map

## Purpose

This phase map turns the human-owned [`ROADMAP.md`](ROADMAP.md) into an implementation sequence.

The phase map may change as Architecture Decision Records (ADRs) add evidence or alter implementation order. An ADR may revise a phase, dependency, or exit gate. It cannot silently change governance, invariants, or roadmap outcomes.

## Change Rules

- Keep completed phase history intact.
- Record a phase-order or exit-gate change in an ADR.
- Add the ADR identifier to the affected phase.
- Do not mark a phase complete without its exit evidence.
- Do not begin work that needs authority reserved for a later phase.
- Treat release versions as targets, not proof of completion.

## Phase Sequence

### Phase 1: Project Foundation (`v0.0.0`)

**Status:** Complete as of 2026-08-18.

**Goal:** Establish the rules and structure that all later work must follow.

**Work:**

- constitutional governance
- architecture boundary
- invariant register
- coding and documentation standards
- roadmap and phase map
- ADR location and template
- minimum repository and validation structure

**Exit gate:** The governing documents agree on what AEGIS protects, what it may decide, and what remains outside the product.

**Exit evidence:** The governing-document review in [`TASKS.md`](TASKS.md) records agreement on the protected repository, explicit authority, deterministic decisions, fail-closed behavior, human escalation, and the software-design boundary. The read-only foundation validator and its focused tests verify required files and the initial changelog structure.

**ADR process:** [`adr/README.md`](adr/README.md) defines the canonical index and [`adr/0000-template.md`](adr/0000-template.md) defines the record template.

**ADR references:** None. Phase 1 establishes the ADR process without making a new architectural decision.

### Phase 2: Authority Model (`v0.1.0`)

**Goal:** Represent human-granted authority without inference.

**Work:**

- task-scope schema
- capability model
- protected-action definitions
- policy ownership and versioning
- validation for missing, conflicting, and unauthorized authority

**Exit gate:** The same scope and policy input produces the same validated authority set. The model cannot enlarge it.

**ADR references:** Add for scope, capability, and policy-format decisions.

### Phase 3: Decision Core (`v0.2.0`)

**Goal:** Decide whether a proposed action has authority.

**Work:**

- deterministic decision pipeline
- `ALLOW`, `DENY`, `STOP`, and `ESCALATE` outcomes
- reason codes
- invariant-check interface
- fail-closed handling for incomplete evidence

**Exit gate:** Table-driven tests reproduce every decision from recorded inputs and reason codes.

**ADR references:** Add for outcome semantics and decision ordering.

### Phase 4: Repository Mediation (`v0.3.0`)

**Goal:** Place supported repository-affecting actions behind one enforced path.

**Work:**

- repository read, write, and creation adapters
- path normalization and boundary checks
- checked-action and executed-action matching
- bypass prevention for the supported deployment profile

**Exit gate:** Supported repository changes cannot occur through the model-facing path without a decision from AEGIS.

**ADR references:** Add for repository adapter and process-isolation decisions.

### Phase 5: Boundary Enforcement (`v0.4.0`)

**Goal:** Detect direct, protected expansion events before execution.

**Work:**

- writes outside task scope
- protected-path changes
- new file or module creation
- dependency-manifest changes
- schema and migration changes
- public-interface changes
- configured permission and policy changes

**Exit gate:** Each supported event has direct evidence, a stable reason code, allowed and blocked tests, and the required stop or escalation result.

**ADR references:** Add for each boundary type and repository-specific signature.

### Phase 6: Evidence (`v0.5.0`)

**Goal:** Make every repository-affecting decision attributable and reviewable.

**Work:**

- decision and execution record format
- task, policy, capability, check, and result correlation
- separation of observed facts from model explanations
- correction or superseding record behavior
- evidence retention and access boundaries

**Exit gate:** A reviewer can reconstruct why an action was allowed, blocked, or escalated without relying on model memory.

**ADR references:** Add for evidence format, retention, and integrity decisions.

### Phase 7: Human Elevation (`v0.6.0`)

**Goal:** Return control to a human when current authority is insufficient.

**Work:**

- narrow elevation request format
- human approval and rejection flow
- approval scope and lifetime
- stale-state revalidation
- approval evidence correlation

**Exit gate:** Approval authorizes only the named action or capability. Unrelated scope remains unchanged.

**ADR references:** Add for approval identity, lifetime, and revalidation decisions.

### Phase 8: Failure Containment (`v0.7.0`)

**Goal:** Preserve authoritative repository state when AEGIS cannot establish authority safely.

**Work:**

- missing and conflicting policy failures
- required parser and evidence failures
- interrupted execution handling
- partial-operation recovery
- corrupted-state detection
- safe operator diagnostics

**Exit gate:** Fault-injection tests show that a required-control failure cannot become an authorized repository change.

**ADR references:** Add for transaction, recovery, and state-integrity decisions.

### Phase 9: End-to-End Integration (`v0.8.0`)

**Goal:** Operate the complete authority path in one supported local deployment profile.

**Work:**

- task creation and policy loading
- model action request
- decision and boundary checks
- mediated repository execution
- evidence review
- elevation and blocked-action recovery

**Exit gate:** End-to-end tests cover allowed, denied, ambiguous, elevated, interrupted, and recovered operations.

**ADR references:** Add for integration boundaries and supported deployment choices.

### Phase 10: Production Candidate (`v0.9.0`)

**Goal:** Produce a release candidate for the declared production profile.

**Work:**

- interface and policy-schema freeze
- security review
- compatibility validation
- upgrade and rollback validation
- operator and maintainer procedures
- supported-platform packaging
- known-limitations register

**Exit gate:** Every `v1.0.0` production gate has traceable evidence or a documented release blocker.

**ADR references:** Add for compatibility, packaging, upgrade, and support decisions.

### Phase 11: Production Use (`v1.0.0`)

**Goal:** Release AEGIS for the declared and documented production use case.

**Work:**

- close all production blockers
- approve release evidence
- publish immutable release artifacts
- publish operator, maintainer, recovery, and limitation documentation
- establish supported maintenance and incident procedures

**Exit gate:** An authorized human confirms that every roadmap production gate is satisfied for the declared deployment profile.

**ADR references:** Add final release and support-boundary decisions.

## Post-Production Phases (`v1.x.0`)

Add a new numbered phase for each approved `v1.x.0` outcome.

Each new phase must state:

- the authority problem it solves
- the roadmap outcome it supports
- the invariants it preserves
- the deterministic evidence it uses
- the supported deployment impact
- the exit gate
- the governing ADRs

A useful capability remains outside AEGIS when it judges software design instead of controlling repository authority.
