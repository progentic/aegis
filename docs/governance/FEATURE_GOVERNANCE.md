# Feature Governance

## Purpose

This document prevents AEGIS from growing into an AI supervisor, autonomous engineering platform, or second source of software-design intelligence.

AEGIS must remain a gateway that controls authority between an LLM and a repository.

## Primary Admission Test

Every proposed feature must answer:

> Does this feature control what the LLM is allowed to do, or is it trying to decide what the LLM should think?

If the feature primarily decides what the LLM should think, it does not belong in AEGIS.

## Required Feature Questions

Before an AEGIS feature is accepted, reviewers should ask:

1. What repository or execution authority does this feature govern?
2. What concrete risk exists without it?
3. Can the enforcement decision be made from deterministic facts?
4. Does the feature reduce risk more than it increases operator and implementation complexity?
5. Could the same capability live in the model, CI, developer tooling, observability, or another system instead?
6. If the feature were removed, would AEGIS lose a meaningful authority-control capability?

A feature that cannot answer these questions should not be added.

## Features That Naturally Belong in AEGIS

Examples include:

- filesystem allowlists and denylists
- task-scoped write permissions
- capability leases
- protected-directory enforcement
- secret-access restrictions
- network or process capability restrictions
- repository invariant enforcement
- deterministic detection of boundary expansion
- explicit elevation workflows
- immutable or tamper-evident execution records
- policy provenance

These mechanisms control authority.

## Features That Normally Belong Elsewhere

Examples include:

- autonomous refactoring
- code-quality optimization
- architectural recommendations
- semantic judgments about whether an abstraction is appropriate
- LLM judges that approve design choices
- embedding-based determinations of architectural correctness
- Janitor agents
- production profiling used to select refactors
- mutation testing as an authorization oracle
- scoring whether code is elegant, simple, or maintainable

These may be valuable capabilities. Their value does not make them gateway responsibilities.

They should normally live in:

- the model
- CI
- static analysis
- developer tooling
- observability
- maintenance automation
- a separate agent system

## Explicit Externalization Rules

The following capabilities are intentionally outside AEGIS. They may be useful elsewhere, but they do not control repository authority.

### AST Accounting and Weighted Node Scoring

AEGIS must not score implementation quality using AST growth, weighted node deletion, function or class deletion bonuses, rolling AST budgets, or similar complexity metrics.

If smaller implementations are desired, the model should be instructed to prefer deletion, reuse, and narrow changes. The gateway should enforce scope rather than score style.

### Semantic Duplicate Detection

AEGIS must not use embedding similarity thresholds to decide that model-produced code duplicates existing code.

If a model cannot reliably discover canonical utilities, repository discovery should be improved through documentation, explicit registries, naming, deterministic metadata, or task context. A probabilistic duplicate detector must not be added to the authority path to compensate for poor discovery.

### Mutation-Test Scoring

AEGIS must not use mutation testing or mutation scores as an authorization oracle. Test quality belongs to CI and engineering review.

### Deep Impact and Dependency Analysis

AEGIS should use direct file, manifest, schema, export, protected-path, and configured signature checks when those facts are sufficient to establish a boundary event.

Deep impact graphs, dependency-cruiser complexity analysis, broad dependency graphs, or other architecture-analysis machinery do not belong in the gateway merely to judge change quality.

### LLM Judges and Embedding Thresholds

AEGIS must not place an LLM, embedding threshold, probabilistic classifier, or realism judge in the authorization path to supervise another LLM.

Adding another probabilistic intelligence layer does not create deterministic authority.

### Autonomous Maintenance

Janitor agents, autonomous refactoring, code cleanup, architectural optimization, and similar maintenance behaviors are agent capabilities, not gateway capabilities.

### Production Observability and Dead-Code Decisions

Datadog profiling, eBPF execution maps, production usage analysis, and dead-code decisions based on production telemetry belong to observability and maintenance systems.

AEGIS records its own control-plane decisions. It does not become an application observability platform or decide whether production code should be removed.

### Cross-PR Complexity Tracking

AEGIS must not maintain rolling complexity windows, cross-PR AST ledgers, or architectural trend scores as a substitute for narrow task definition.

If a task is producing excessively broad changes, task scope and capabilities should be tightened.

### Auto-Fix and Patch Generation

AEGIS must not author, rewrite, or auto-fix model code. It may mechanically apply a model-produced change after authority checks succeed.

Code generation belongs to the model or developer tooling. Authority enforcement belongs to AEGIS.

## Why Externalization Matters

AEGIS exists partly because model-generated changes can already impose a substantial review burden. It must not move that burden into a more complex gateway.

The intended relationship is:

```text
LLM complexity
      |
      v
simple deterministic constraints
      |
      v
smaller, more predictable repository changes
```

The architecture must avoid:

```text
LLM complexity + gateway complexity + PR complexity = worse
```

The gateway should become more capable only when added capability materially improves authority control.

## Probabilistic Systems

A probabilistic component must not silently grant repository authority.

If a probabilistic tool is integrated for informational purposes, its output must remain advisory unless a human converts that information into explicit authorization.

Examples:

```text
Allowed:
"Similarity tool reports a possible duplicate. Human or model may inspect it."

Not allowed:
"Similarity score is 0.91, therefore repository write authority is denied or granted."
```

Deterministic policy may deny an action because a human explicitly configured a deterministic rule. It must not disguise a probabilistic software-design judgment as an authority fact.

## Complexity Budget for the Gateway Itself

AEGIS exists partly to reduce the review and governance burden created by LLM-generated changes.

It must not move that bottleneck into the gateway.

Therefore:

- every abstraction must earn its existence
- every feature must have a clear authority-control purpose
- the trusted enforcement path should remain small
- repository-specific intelligence should be policy data where possible, not new reasoning subsystems
- optional analysis should remain outside the core authorization path
- removing a feature should be preferred when it does not materially weaken authority enforcement

## Prohibited Architectural Direction

AEGIS must not develop a decision branch equivalent to:

> AEGIS reasons about whether this is probably a good idea.

If such a branch is required, the capability is performing software-design intelligence rather than authority enforcement.

That capability belongs outside AEGIS.

## Review Outcome

Feature review should produce one of three results:

- **ADMIT** — necessary to control authority and consistent with the constitutional rules.
- **EXTERNALIZE** — useful capability, but belongs in another system.
- **REJECT** — adds complexity without a meaningful authority-control benefit or violates AEGIS governance.

## Governance Precedence

No feature approval may override the constitutional rules in the repository root `GOVERNANCE.md`.
