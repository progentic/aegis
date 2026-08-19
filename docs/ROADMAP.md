# AEGIS Roadmap

## Control

This roadmap is the human-owned release sequence for AEGIS.

Agents must treat this file as immutable unless a task explicitly authorizes a roadmap change. Implementation progress, ADRs, test results, or model recommendations do not alter roadmap authority.

The roadmap defines release outcomes. [`PHASEMAP.md`](PHASEMAP.md) may revise the work sequence used to reach them.

## Release Sequence

| Version | Milestone | Required outcome |
|---|---|---|
| `v0.0.0` | Bootstrap | Establish the repository structure, constitutional governance, architecture boundary, documentation rules, invariant register, coding standard, and ADR process. |
| `v0.1.0` | Authority Contract | Define machine-readable task scope, capabilities, protected actions, policy ownership, and explicit authorization sources. |
| `v0.2.0` | Deterministic Decision Core | Produce reproducible `ALLOW`, `DENY`, `STOP`, and `ESCALATE` results from task scope, policy, capabilities, and direct repository facts. |
| `v0.3.0` | Repository Mediation | Route supported repository reads, writes, and file creation through one enforced mediation path. Prevent direct repository-affecting bypass. |
| `v0.4.0` | Boundary Enforcement | Detect configured path, dependency, schema, public-interface, permission, and protected-policy changes. Stop or escalate actions without authority. |
| `v0.5.0` | Evidence Record | Record attributable authority decisions, checks, boundary events, execution results, and human approval references. Keep model explanations separate from observed facts. |
| `v0.6.0` | Human Elevation | Present narrow elevation requests and apply human decisions without enlarging unrelated task scope. |
| `v0.7.0` | Failure Containment | Fail closed when policy, evidence, parsers, or enforcement components are missing, conflicting, corrupted, or unavailable. |
| `v0.8.0` | End-to-End Integration | Complete one supported local workflow from human-approved task scope through decision, mediated execution, evidence recording, and recovery from a blocked action. |
| `v0.9.0` | Production Candidate | Freeze required interfaces, resolve release blockers, validate upgrade and recovery paths, and complete security and operational review for the supported deployment profile. |
| `v1.0.0` | Production Use | Release a documented, supportable authority gateway whose declared deployment profile passes the production gates below. |
| `v1.x.0` | Planned | Add compatible, human-approved authority controls and repository integrations without weakening the `v1.0.0` invariants or expanding AEGIS into software-design judgment. |

## Production Gates for `v1.0.0`

`v1.0.0` requires evidence that:

- supported repository actions cannot bypass the Repository Mediator
- authorization decisions are deterministic and reproducible
- missing or conflicting authority fails closed
- protected boundary changes stop or escalate before execution
- task scope and approvals cannot expand themselves
- evidence records connect each request, decision, approval, and execution result
- recovery procedures preserve authoritative repository state
- operator and maintainer documentation is complete for the supported deployment profile
- security, compatibility, upgrade, and rollback tests pass on every supported platform
- known limitations and unsupported actions are documented

Passing a test suite alone does not satisfy these gates. Release evidence must show that the authority boundary works under allowed, denied, ambiguous, and failed conditions.

## `v1.x.0` Change Boundary

Planned `v1.x.0` work may include:

- additional deterministic repository adapters
- additional human-approved capability types
- clearer elevation and evidence interfaces
- compatible policy-schema extensions
- operational improvements to supported deployment profiles

Planned work must not add:

- LLM judges
- probabilistic authorization
- autonomous code repair or refactoring
- software-quality or architecture scoring
- production observability used to make design decisions
- automatic expansion of task scope or permissions

Any release that changes an invariant, authority source, failure model, or compatibility contract requires explicit human roadmap approval and an ADR.
