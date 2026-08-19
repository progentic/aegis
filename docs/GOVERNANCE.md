# AEGIS Governance

## Purpose

AEGIS is a gateway between an LLM and an authoritative software repository.

The object being protected is the **repository**.

AEGIS does not exist to maximize code quality, test coverage, architectural elegance, or autonomous software-engineering capability. It exists to prevent an LLM from moving unauthorized, out-of-scope, or boundary-expanding changes into authoritative repository state without explicit human authorization.

An LLM may produce code that is syntactically valid and test-passing while still being architecturally wrong, outside the intended task, or based on an invented implementation direction. AEGIS constrains what the model is allowed to do before that output reaches the repository.

## Constitutional Status

This document defines the constitutional rules for AEGIS.

These rules are above implementation details, runtime configuration, feature proposals, policy profiles, and subsystem design. A lower-level policy must not weaken or reinterpret them.

If a conflict exists, this document takes precedence.

## The Three Constitutional Rules

### 1. AEGIS Governs Authority, Not Software Design

AEGIS does not decide how software should be built.

AEGIS ensures that an LLM can build only within boundaries humans have already authorized.

Architecture decisions belong to humans. Software reasoning belongs to the model. Authority enforcement belongs to AEGIS.

AEGIS must not decide whether an architectural choice is good, elegant, maintainable, clever, or desirable.

### 2. Uncertainty Returns Control to the Human

When AEGIS cannot deterministically establish that an action is permitted, it returns authority to the human.

AEGIS must not reason around uncertainty. It must not use probabilistic judgment as a fallback for authorization.

If evidence is insufficient, scope is ambiguous, policy is incomplete, or an action cannot be deterministically classified, AEGIS stops or escalates according to policy.

### 3. Authority Must Be Explicitly Granted

Permission is affirmative, not inferred.

Absence of authorization is denial. Silence is not consent. Incomplete policy is not authorization.

Examples:

- If policy does not list a file as writable, it is not writable.
- If task scope does not authorize a new module, a new module is not authorized.
- If policy does not explicitly authorize a schema change, the schema change requires elevation.
- If evidence is insufficient, AEGIS escalates.
- If scope is ambiguous, AEGIS escalates.

AEGIS must never infer that a human "probably meant" to authorize an action.

## Separation of Concerns

The model is responsible for intelligence:

- understand
- plan
- reason
- write
- debug
- refactor

AEGIS is responsible for authority:

- allow
- deny
- constrain
- verify boundaries
- record
- escalate

This separation is a primary architectural invariant.

AEGIS must not become an AI supervisor that attempts to determine what the model should think.

## What AEGIS Answers

AEGIS answers six questions only:

1. Is the LLM allowed to perform this action?
2. Is the action within the human-approved task scope?
3. Does the action violate a known repository invariant?
4. Is the action trying to expand scope, permissions, dependencies, or architecture?
5. Is there sufficient deterministic evidence to permit the change?
6. If not, should execution stop and return control to the human?

These questions define the boundary of the product.

## The Feature-Creep Test

Every proposed AEGIS feature must answer:

> Does this feature control what the LLM is allowed to do, or is it trying to decide what the LLM should think?

If the feature is primarily trying to decide what the model should think, it does not belong in AEGIS.

A capability may be useful and still be outside AEGIS.

Examples that require special scrutiny include:

- embedding-based architectural judgment
- another LLM acting as a judge
- autonomous refactoring
- architectural recommendations
- code-quality scoring
- Janitor agents
- production profiling used to make design decisions
- probabilistic determinations of whether an implementation is "good"

The first question for any such proposal is:

> Does AEGIS need this capability to control authority?

If not, it belongs in the model, CI, developer tooling, observability, or another system.

## Deterministic Core

Security-relevant authorization decisions must be based on deterministic facts available to AEGIS.

Examples include:

- file paths
- explicit task scope
- configured capabilities
- diff contents
- exported-symbol changes
- dependency-manifest changes
- protected-directory writes
- repository policy
- machine-readable invariants
- explicit human approvals

Probabilistic systems may exist outside the authorization path, but they must not silently grant authority or convert an unauthorized action into an authorized one.

## Architecture Expansion

Architecture expansion is detected as **observable events, not interpretation**.

AEGIS does not infer whether the model is making a bad architectural decision. It detects concrete boundary changes and escalates them.

The normative definitions are maintained in [governance/BOUNDARY_GOVERNANCE.md](governance/BOUNDARY_GOVERNANCE.md).

## Default Decision Model

The default decision model is:

```text
LLM proposes action
        |
        v
Is action explicitly authorized?
   |                 |
  no                yes
   |                 |
 STOP                v
              Within task scope?
               |            |
              no           yes
               |            |
           ESCALATE         v
                    Violates known invariant?
                       |             |
                      yes           no
                       |             |
                      STOP           v
                           Expands protected boundary?
                              |             |
                             yes           no
                              |             |
                          ESCALATE          v
                              Sufficient deterministic evidence?
                                   |                |
                                  no               yes
                                   |                |
                               ESCALATE            ALLOW
```

There is deliberately no branch that says:

> AEGIS reasons about whether this is probably a good idea.

That branch must never exist.

## Governance Documents

The governance model is intentionally split by responsibility:

- [governance/AUTHORITY_GOVERNANCE.md](governance/AUTHORITY_GOVERNANCE.md) — capabilities, task scope, explicit authorization, and repository authority.
- [governance/BOUNDARY_GOVERNANCE.md](governance/BOUNDARY_GOVERNANCE.md) — deterministic definitions of architecture and scope expansion.
- [governance/ESCALATION_GOVERNANCE.md](governance/ESCALATION_GOVERNANCE.md) — stop, deny, escalate, human elevation, and uncertainty handling.
- [governance/FEATURE_GOVERNANCE.md](governance/FEATURE_GOVERNANCE.md) — admission rules for new AEGIS features and controls against gateway feature creep.

## Simplicity Requirement

AEGIS must not solve LLM complexity by creating a more complex gateway bottleneck.

Every abstraction must earn its existence.

Every feature must reduce execution risk more than it increases cognitive and operational load.

AEGIS is not intended to become the most feature-rich AI gateway. It is intended to remain a clear, safe, predictable authority boundary between an LLM and the repository.
