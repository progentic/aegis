# Boundary Governance

## Purpose

This document defines architecture and scope expansion as deterministic, observable repository events.

AEGIS does not decide whether an architectural decision is good or bad. It detects whether the model is crossing a boundary that humans have not already authorized.

## Core Rule

Architecture expansion is an observable event, not an interpretation.

If AEGIS detects an expansion event that is not explicitly authorized, it does not evaluate the merit of the change. It escalates.

## Observable Expansion Events

### New Module

**Event:** A file representing a new module or source unit appears outside the task's authorized creation scope.

**Deterministic evidence:** Repository path did not exist before the proposed change and is not authorized for creation.

**Default action:** Escalate.

### New Public Interface

**Event:** The diff adds a new exported or public symbol.

Examples include:

- `pub fn`
- `pub struct`
- `pub enum`
- `pub type`
- exported JavaScript or TypeScript symbols
- public interfaces in another supported language

**Deterministic evidence:** Export/public-surface diff.

**Default action:** Escalate unless explicitly authorized by task policy.

### New Dependency

**Event:** A dependency manifest or lock-relevant dependency declaration is changed to add or alter a dependency.

Common manifests include:

- `Cargo.toml`
- `package.json`
- `go.mod`
- `requirements.txt`

**Deterministic evidence:** Manifest diff.

**Default action:** Escalate.

### New or Modified Schema

**Event:** A schema-bearing file or migration path is changed.

Common examples include:

- `migrations/**`
- `*.sql`
- `schema.prisma`
- `schema.rs`

**Deterministic evidence:** Protected path or schema-file diff.

**Default action:** Escalate unless schema authority is explicit.

### New Service Connection

**Event:** The diff introduces a new external or inter-service connection mechanism.

Observable indicators may include configured, repository-specific patterns for:

- a new network client import
- a new queue topic declaration
- a new gRPC client import or generated client binding
- a new service endpoint configuration

**Deterministic evidence:** Human-defined import, path, configuration, or manifest patterns.

**Default action:** Escalate.

AEGIS must not infer service intent from natural-language code meaning. Detection rules must be defined by deterministic repository facts.

### New Permission

**Event:** A file governing authorization, roles, policy, or permissions is changed.

Common examples include:

- authorization source files
- role definitions
- policy files
- permission JSON or YAML

**Deterministic evidence:** Protected path or configured policy-file diff.

**Default action:** Escalate.

### Protected-Directory Write

**Event:** The model attempts to write to a protected directory without explicit authorization.

Common protected directories include:

- `.github/`
- `policy/`
- `aegis/`
- `migrations/`

**Deterministic evidence:** Target path.

**Default action:** Deny or escalate according to repository policy.

### Modification Outside Authorized Scope

**Event:** The proposed change modifies a file not included in the task's allowed write set.

**Deterministic evidence:** File path compared with active task authorization.

**Default action:** Stop or escalate.

## No Architectural Judgment

AEGIS must not attempt to determine:

- whether a new module is a good abstraction
- whether a dependency is the best library
- whether a public API is elegant
- whether a schema design is maintainable
- whether a service boundary is conceptually correct
- whether a refactor is architecturally preferable

Those questions require software-design intelligence and remain outside AEGIS.

## Direct Detection Requirement

Boundary detection should use the smallest direct deterministic fact that establishes whether authority is present.

Preferred inputs include:

- file paths and globs
- file creation or deletion facts
- manifest diffs
- schema-bearing path diffs
- exported-symbol diffs
- configured protected paths
- human-defined import or configuration signatures
- explicit task authorization

AEGIS should not construct deep dependency or impact graphs merely to determine whether a change is architecturally wise. If a direct path or diff fact is sufficient, deeper analysis is outside the gateway boundary.

Dependency and impact analysis may exist in CI or developer tooling. It does not become an authority source unless a separate deterministic repository rule explicitly maps an observable fact to a protected action.

## Repository-Specific Detection

Projects may extend the observable-event catalog with additional deterministic boundary rules.

Each added rule must define:

1. The observable repository event.
2. The deterministic evidence used to detect it.
3. The authority required to permit it.
4. The default action when authority is absent.

A rule that requires AEGIS to infer design quality, intent, semantic elegance, or probability of correctness is invalid under this governance model.

## Boundary Event Output

When AEGIS detects an unauthorized expansion event, it should report the fact without making a design judgment.

Example:

```text
BOUNDARY_EXPANSION_DETECTED
Type: NEW_DEPENDENCY
File: Cargo.toml
Task authorization: dependency changes not granted
Action: ESCALATE
```

It should not report:

```text
The new dependency appears unnecessary or architecturally unwise.
```

The first statement is authority enforcement. The second is architectural judgment.
