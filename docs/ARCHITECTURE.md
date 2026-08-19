# AEGIS Architecture

## Purpose

AEGIS is a constrained mediation layer between an LLM and an authoritative software repository.

The repository is the object being protected.

AEGIS exists because an LLM can produce code that is syntactically valid, compiles, and passes tests while still being outside the requested task, inconsistent with repository invariants, or based on an invented implementation direction. AEGIS reduces that risk by controlling what the model is permitted to do before repository state is changed.

AEGIS does **not** decide how software should be built. It enforces authority that humans have already defined.

The constitutional rules governing this architecture are defined in [`GOVERNANCE.md`](GOVERNANCE.md). Where this document and governance conflict, governance takes precedence.

## Design Objective

AEGIS should reduce the uncertainty introduced by model-driven repository changes without becoming another source of engineering complexity.

The intended flow is:

```text
LLM complexity
      |
      v
simple deterministic constraints
      |
      v
smaller, more predictable repository changes
```

AEGIS must not replace one bottleneck with another:

```text
LLM complexity + gateway complexity + PR complexity = worse
```

The gateway therefore remains intentionally narrow. Intelligence stays with the model. Authority stays with AEGIS.

## System Boundary

```text
Human intent and authorization
            |
            v
           LLM
            |
            | proposed actions / code / commands
            v
+---------------------------------------------+
|                    AEGIS                    |
|                                             |
|  Task Scope                                 |
|  Capability & Permission Enforcement        |
|  Repository Invariant Enforcement           |
|  Observable Boundary Detection              |
|  Deterministic Decision Engine              |
|  Evidence Recording                         |
|  Human Escalation                           |
+----------------------+----------------------+
                       |
                       | authorized actions only
                       v
              Authoritative Repository
```

The LLM must not possess a repository-affecting path that bypasses AEGIS.

AEGIS may mechanically apply an authorized model-produced change. It must not author, rewrite, improve, or auto-fix code on the model's behalf.

## Separation of Responsibilities

### Model Responsibilities

The model provides software-engineering intelligence:

- understand the task
- inspect repository context
- plan an implementation
- reason about design
- write code
- debug failures
- refactor within authorized scope
- explain why elevation may be necessary

### AEGIS Responsibilities

AEGIS controls authority:

- allow
- deny
- constrain
- verify task and repository boundaries
- record evidence
- escalate to a human

AEGIS does not judge whether the model's idea is elegant, maintainable, optimal, realistic, or likely to be correct.

## Core Architectural Components

AEGIS should contain only the components necessary to enforce repository authority.

### 1. Task Scope Loader

The Task Scope Loader receives the human-approved execution boundary for the current task.

A task scope identifies what the model may access or change. At minimum, it should be capable of expressing:

- allowed read paths
- allowed write paths
- allowed file creation paths
- protected actions explicitly authorized for the task
- repository invariants applicable to the task
- granted process, network, secret, or other execution capabilities when such capabilities are supported

Example:

```yaml
task_id: FIX-214
read:
  - src/auth/**
  - tests/auth/**
write:
  - src/auth/token.rs
  - tests/auth/token_test.rs
create: []
protected_actions: []
```

The scope is authority data. The model cannot enlarge it.

### 2. Capability and Permission Engine

The Capability and Permission Engine answers whether the model is permitted to perform a requested action.

Authority is explicit. A capability may cover actions such as:

- reading a path
- writing a path
- creating a file
- invoking an allowed process
- accessing a secret
- reaching an allowed network destination
- performing a protected repository action

Capabilities should be narrow, attributable, revocable, and scoped to the current task or lease where practical.

The engine does not infer permission from model intent, repository convention, passing tests, or previous unrelated approvals.

### 3. Repository Invariant Enforcer

Repository invariants are human-defined facts that AEGIS may enforce deterministically.

Examples:

```text
Generated files cannot be edited directly.
Schema changes require elevation.
Authorization policy files require elevation.
UI code cannot write authoritative state directly.
Dependency manifests require explicit authorization.
```

AEGIS enforces the invariant as written. It does not invent new invariants based on its interpretation of the code.

### 4. Observable Boundary Detector

The Boundary Detector identifies concrete repository events that represent scope or architecture expansion.

Its job is detection, not evaluation.

The normative definitions are maintained in [`governance/BOUNDARY_GOVERNANCE.md`](governance/BOUNDARY_GOVERNANCE.md).

Typical events include:

| Event | Deterministic evidence |
|---|---|
| New module | New file or source unit outside authorized creation scope |
| New public interface | Added exported/public symbol |
| New dependency | Dependency manifest change |
| New schema | Migration or schema-bearing file change |
| New service connection | Human-defined import/config/path signature |
| New permission | Authz, role, permission, or policy file change |
| Protected-directory write | Target path matches protected path |
| Modification outside scope | Changed path is absent from allowed write set |

When an unauthorized event is found, AEGIS reports the boundary expansion and escalates or stops according to policy.

AEGIS does not determine whether the expansion is a good idea.

### 5. Deterministic Decision Engine

The Decision Engine applies task scope, capabilities, repository invariants, and observable boundary facts.

Its authority outcomes are defined by [`governance/ESCALATION_GOVERNANCE.md`](governance/ESCALATION_GOVERNANCE.md):

- `ALLOW`
- `DENY`
- `STOP`
- `ESCALATE`

The decision path is deliberately small:

```text
LLM proposes action
        |
        v
Explicitly authorized?
   |             |
  no            yes
   |             |
 STOP            v
           Within task scope?
              |       |
             no      yes
              |       |
         ESCALATE     v
              Violates known invariant?
                    |       |
                   yes      no
                    |       |
                   STOP     v
                 Protected boundary expansion?
                         |       |
                        yes      no
                         |       |
                    ESCALATE     v
                    Sufficient deterministic evidence?
                              |       |
                             no      yes
                              |       |
                         ESCALATE   ALLOW
```

There is no branch in which AEGIS reasons about whether an implementation is probably a good idea.

### 6. Repository Mediator

The Repository Mediator is the only repository-affecting execution path available to the model.

It performs authorized operations after the Decision Engine permits them.

Responsibilities include:

- enforcing the active path and capability set
- preventing writes outside the approved boundary
- applying authorized model-produced changes
- preventing direct access to protected repository state without authority
- rejecting operations that do not match the granted capability

The mediator is not a code generator. It must not synthesize fixes or alter model output to make a blocked change acceptable.

### 7. Evidence Recorder

Every repository-affecting decision should produce an evidence record sufficient to explain what authority was used and what happened.

A minimal record may include:

```text
Task ID
Model/session identifier
Policy version
Requested action
Authorized paths/capabilities
Files read or modified
Boundary events detected
Invariant checks performed
Decision: ALLOW | DENY | STOP | ESCALATE
Human elevation reference, if any
Result of authorized execution
```

Evidence records distinguish deterministic gateway facts from model-provided explanations.

AEGIS records what happened. It does not declare that the resulting software is correct.

### 8. Human Elevation Interface

Elevation returns authority to a human when existing authorization is insufficient.

An elevation request should contain observed facts, not an AEGIS design recommendation.

Example:

```text
BOUNDARY_EXPANSION_DETECTED
Task: FIX-214
Type: NEW_DEPENDENCY
File: Cargo.toml
Current authority: dependency changes not granted
Requested action: modify dependency manifest
Model reason: <model-provided explanation>
Action: HUMAN_DECISION_REQUIRED
```

A human may approve a narrow extension of authority or reject it.

Approval applies only to the specified capability or action. It does not implicitly enlarge unrelated task scope.

## Request Lifecycle

A normal repository-affecting operation follows this sequence:

1. A human creates or approves task scope.
2. AEGIS derives the active capabilities from human-owned policy and task scope.
3. The model reads permitted context and proposes an action.
4. AEGIS checks explicit permission.
5. AEGIS checks task scope.
6. AEGIS checks known repository invariants.
7. AEGIS checks direct observable boundary events.
8. AEGIS determines whether sufficient deterministic evidence exists.
9. AEGIS allows, denies, stops, or escalates.
10. If allowed, the Repository Mediator performs the authorized action.
11. AEGIS records the decision and execution evidence.

At no stage does AEGIS substitute its own software-design judgment for the model or human.

## Architecture Expansion: Direct Facts Only

Boundary enforcement should use the smallest direct fact that answers the authority question.

Preferred inputs include:

- exact or globbed file paths
- file existence before and after a proposed change
- configured protected paths
- manifest diffs
- schema-bearing path diffs
- exported-symbol diffs
- configured repository-specific import or configuration signatures
- explicit task authorization
- explicit human approval

AEGIS should not construct deep impact graphs or broad semantic models when a direct path or diff check can answer the authority question.

If a proposed rule requires significant inference about software meaning, it is likely outside the gateway boundary.

## What Is Deliberately Outside AEGIS

The detailed feature-admission rules are normative in [`governance/FEATURE_GOVERNANCE.md`](governance/FEATURE_GOVERNANCE.md). Architecturally, the following capabilities are externalized because they attempt to improve or judge software rather than control authority.

| Capability | Why it is outside AEGIS | Appropriate home |
|---|---|---|
| AST accounting and weighted node-deletion scoring | Judges implementation complexity and encourages particular coding behavior | Agent instructions, review tooling |
| Semantic duplicate detection with embeddings | Uses probabilistic similarity to judge implementation direction | Agent discovery tooling, developer tooling |
| Mutation-test scoring | Measures test quality rather than repository authority | CI |
| Deep impact graphs and dependency complexity analysis | Adds engineering-analysis machinery beyond direct authority facts | CI, developer tooling, architecture analysis |
| Embedding thresholds | Probabilistic judgment is not an authority fact | External advisory tooling |
| LLM realism or design judges | Adds another probabilistic intelligence layer to supervise the first | Model workflow or human review |
| Janitor agents and autonomous cleanup | Performs software engineering and maintenance | Agent system |
| Autonomous refactoring and architectural optimization | Decides how software should be built | Model or developer workflow |
| Production profiling, Datadog, eBPF execution maps | Determines runtime behavior and operational health | Observability platform |
| Dead-code decisions from production telemetry | Requires interpretation of operational evidence | Maintainers, observability, maintenance tooling |
| Seven-day complexity windows and cross-PR complexity tracking | Scores engineering trends instead of controlling current task authority | Review analytics, engineering metrics |
| Auto-fix patches | Authors code rather than mediating authority | Model, IDE, developer tooling |

These capabilities may be useful. Their usefulness does not make them gateway responsibilities.

## Canonical Discovery

AEGIS may enforce an explicitly defined repository rule about use of a canonical location or approved interface when that rule is machine-readable and deterministic.

AEGIS should not use embeddings to discover that two implementations are semantically similar.

If models repeatedly create duplicate utilities because canonical code cannot be found, the preferred remediation is to improve repository discovery for the model, such as:

- clearer repository documentation
- explicit canonical registries
- better naming
- task context that points to approved utilities
- deterministic repository metadata

Discovery is an input-quality problem. Probabilistic duplicate judgment should not be moved into the authority gateway.

## Testing Boundary

AEGIS may require that a human-owned policy permits a command or workflow to run. It may record whether an allowed validation command succeeded.

AEGIS does not determine whether test coverage is sufficient, whether tests are meaningful, or whether a mutation score proves correctness.

Those are CI and engineering-review responsibilities.

A passing test suite is evidence about execution outcome. It is not a source of repository authority.

## Observability Boundary

AEGIS records its own authority decisions and execution evidence.

It is not a production observability platform.

AEGIS should not use production profiling, Datadog telemetry, eBPF execution maps, or sampled runtime behavior to decide whether application code is dead, unnecessary, or safe to delete.

Production telemetry may inform humans or external engineering systems. It does not belong in the gateway's authorization path.

## Complexity Boundary

AEGIS does not maintain code-complexity budgets, rolling AST ledgers, or cross-PR architectural scores.

If model-generated changes repeatedly become too large or diffuse, the first control should be tighter human task scope and narrower capabilities.

Examples:

```text
Too many files touched       -> narrow allowed write paths
Unexpected new module        -> creation not authorized; escalate
Dependency added             -> manifest change requires elevation
Schema change                -> schema boundary requires elevation
Unrelated refactor           -> outside task scope; stop or escalate
```

This keeps complexity control at the authority boundary instead of creating a second software-analysis system.

## Failure Model

Repository-affecting actions fail closed.

AEGIS must not modify authoritative repository state when any required authority fact cannot be established because of:

- missing task scope
- missing or conflicting policy
- parser failure for a required deterministic check
- corrupted gateway state
- unavailable required evidence
- enforcement subsystem failure

Failure does not authorize fallback reasoning. The action stops or escalates.

## Architectural Simplicity Rules

The gateway architecture should remain explainable as a small authority-control pipeline.

New components should be rejected when they primarily:

- score software quality
- predict correctness
- recommend architecture
- improve model reasoning
- autonomously modify code
- analyze production application behavior
- accumulate engineering metrics unrelated to current authority

Before adding any component, ask:

> Does this component control what the LLM is allowed to do, or is it trying to decide what the LLM should think?

If it controls allowed actions using deterministic facts, it may belong in AEGIS.

If it tries to improve or judge the model's engineering decisions, it belongs elsewhere.

## Architectural Non-Goals

AEGIS is not:

- an autonomous software-engineering agent
- an LLM judge
- a code-quality platform
- a test-quality platform
- an architecture optimizer
- a duplicate-code intelligence system
- a production observability platform
- an autonomous maintenance system
- a PR complexity scoring system
- an auto-fix engine

AEGIS is the authority boundary between model execution and repository state.

## Governance References

Normative requirements are split by concern:

- [`GOVERNANCE.md`](GOVERNANCE.md) — constitutional rules and product boundary
- [`governance/AUTHORITY_GOVERNANCE.md`](governance/AUTHORITY_GOVERNANCE.md) — explicit authority and capability sources
- [`governance/BOUNDARY_GOVERNANCE.md`](governance/BOUNDARY_GOVERNANCE.md) — deterministic boundary-expansion events
- [`governance/ESCALATION_GOVERNANCE.md`](governance/ESCALATION_GOVERNANCE.md) — allow, deny, stop, escalation, and fail-closed behavior
- [`governance/FEATURE_GOVERNANCE.md`](governance/FEATURE_GOVERNANCE.md) — feature admission and externalization rules

## Architecture Summary

AEGIS should be difficult to misunderstand.

The model thinks. AEGIS constrains.

The model proposes code. AEGIS controls whether the proposed action has authority to affect the repository.

The model may request broader authority. AEGIS does not grant it; a human does.

When AEGIS has deterministic authorization, it permits the action. When it has deterministic prohibition, it blocks the action. When authority is insufficient or unclear, it returns control to the human.

That is the architecture.
