# ADR 0001: Supported Deployment Profile and Threat Model

## Status

Proposed

Recommendations 1-3 were confirmed by the human task instruction on 2026-10-15. The named approving human remains unresolved, so this record is not yet `Accepted`.

## Date

2026-10-15

## Decision Owners

Not yet named. The repository owner must name the approving human before this record becomes `Accepted`.

## Phase and Version

Roadmap `v0.3.0` (repository mediation and executor isolation). This record also sets the trust assumptions that `v0.1.0` and `v0.2.0` must not contradict, and the installation path that `v0.8.0` and `v0.9.0` must deliver.

## Affected Invariants

`INV-001`, `INV-002`, `INV-003`, `INV-007`, `INV-011`, `INV-012`, `INV-013`, `INV-016`.

This record adds trust assumptions. It does not weaken, replace, or reword any invariant.

## Context

AEGIS controls what an LLM may change in a repository. A library check cannot stop another process that has filesystem permission from writing directly. `INV-001` therefore needs an operating-system boundary, and that boundary depends on the platform.

No runtime, executor, or installer exists on `main`. Until this decision is approved, no component can be named as the trusted mediator, and `v0.3.0` cannot be tested against a real boundary.

## Decision

The following records the confirmed direction for recommendations 1-3. Final ADR acceptance still requires the named decision owner. Recommendation 4, naming the approver for this ADR and the Phase 1 acceptance record, remains pending.

| Recommendation | Recorded direction | Status |
| :--- | :--- | :--- |
| 1. Production platform | Linux only; one distribution/version, Ubuntu 24.04 LTS, for the initial profile | Confirmed direction; runtime validation pending |
| 2. Machine/user scope | One human operator on a trusted-admin machine, with three separate service identities | Confirmed |
| 3. Supply-chain assessment | Comprehensive review at `v0.9.0`; pinned inputs and controlled builds from the first runtime implementation | Confirmed |
| 4. Approving human | Not yet named | Pending; blocks final ADR acceptance |

### Adversaries in scope

- Model-generated actions, including shell commands and every process they start.
- Malicious text in repository files, tool output, or model input that tries to widen authority.
- Forged, stale, expired, or replayed permits.
- Attempts by the model's process to change policy, task scope, evidence, or AEGIS configuration.

### Trusted

- The human who approves task scope and elevations.
- The operating-system administrator and the host kernel. AEGIS does not claim to resist a hostile administrator.
- The AEGIS mediator, decision engine, and evidence recorder as shipped.

### Out of scope for `v1.0.0`

- A hostile administrator or kernel compromise.
- Physical access to the machine.
- Hostile local human users other than the authorized operator. The supported deployment has one human operator and a trusted administrator; this does not mean one operating-system identity. The model executor, mediator, and control-state owner remain separate identities.
- Resistance to a compromised trusted AEGIS component. Trusting the shipped components does not defer their basic supply-chain integrity controls; those start with the first runtime implementation, with a comprehensive security review at `v0.9.0` before production release.

### Supported profile

Linux is the only production platform selected for `v1.0.0`.

- **Platform:** Ubuntu 24.04 LTS is the single distribution/version for the initial profile. This matches the existing Linux foundation CI baseline, but passing foundation CI does not establish runtime isolation or production support; the real-executor evidence below is still required.
- **Identities:** three separate operating-system identities: the model executor, the AEGIS mediator, and the control-state owner. The model executor identity has no write access to the authoritative repository, policy, permits, or evidence. It may write a separate disposable staging workspace, which is never automatically synchronized into the authoritative repository.
- **Executor:** model tools run only as the model executor identity, inside a sandbox that includes descendants.
- **Repository access:** the model receives only authorized repository context through read-only access. A read-only mount does not grant permission to read paths outside task scope. Only the mediator identity may write the authoritative repository, including `.git`.
- **IPC:** the model executor submits proposed actions through a local Unix domain socket. The gateway checks the caller's identity from the operating system, not from the message, then evaluates authority. The socket does not expose internal execution operations or accept caller-created decisions, evidence, or permits.
- **Storage:** policy, task scope, permits, and evidence live in locations owned by the control-state identity and not writable by the model executor. Evidence storage is append-only or tamper-evident.
- **Installation:** one supported path. Setup creates the identities and permissions, verifies them, and refuses to start if verification fails.
- **macOS and Windows:** unsupported for production in `v1.0.0`. A Linux virtual machine on either platform is a development route only, not an accepted production deployment.

### Supply-chain controls and review timing

The first runtime implementation must use pinned dependency and build-tool versions, with lockfiles where the selected ecosystem supports them. Changes to these inputs remain human-controlled.

Builds must use reviewed source and a controlled environment outside the model-writable staging workspace. Record the source commit/tree, dependency and tool versions, and artifact checksums so the output is attributable to its inputs.

The comprehensive supply-chain assessment is part of the `v0.9.0` security review and must complete before `v1.0.0` production admission. Deferring that assessment does not permit uncontrolled dependencies or builds during `v0.1.0` through `v0.8.0`.

## Alternatives Considered

- **Library checks only.** Rejected. A process with filesystem permission can bypass them, which `INV-001` forbids.
- **Native macOS with separate user accounts.** Not selected for `v1.0.0`. It adds a separate installation path, operating-system enforcement implementation, and bypass matrix. It can be reconsidered by a later ADR.
- **Containers only, with no separate mediator identity.** Rejected. A container boundary alone does not show that the mediator is the only writer of the repository.
- **Supporting several platforms at once.** Rejected for `v1.0.0`. Every platform needs its own bypass matrix, which delays the first usable release.

## Consequences

- Benefit: one platform means one bypass matrix and one install path, which supports the out-of-the-box goal.
- Benefit: the trusted mediator is a named, testable identity.
- Cost: users on macOS or Windows cannot run a supported production setup in `v1.0.0`.
- Cost: installation needs permission to create identities. The installer must explain this plainly and must not weaken protection if permission is missing.
- Follow-up: the runtime language, packaging, and model-tool interface need their own ADRs, and none may contradict this one.

## Validation Evidence

`v0.3.0` must show, on the supported platform, using the real model tools and their child processes:

- The model identity cannot write the repository, including `.git`, by direct write, rename, symlink, hard link, permission change, or file-descriptor passing.
- The model can write its disposable staging workspace without changing the authoritative repository.
- The model identity cannot read or write policy, scope, permits, or evidence stores.
- A request signed or claimed by the model cannot substitute for the operating-system caller identity.
- Installation fails closed when any identity, permission, or socket check fails.
- Each check produces evidence naming the platform version, source commit, and result.

## Supersedes

None

## Superseded By

None
