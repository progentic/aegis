# Phase 2 Acceptance Record

## Status

**Accepted.** The human instructed: "accept phase 2 and start ADR-0004".
This is explicit acceptance of the implemented authority-contract work at the
reviewed source below. Test results support the record; they do not supply the
human acceptance.

## Reviewed Source

| Field | Value |
| :--- | :--- |
| Phase | Phase 2, Authority Model (`v0.1.0` target) |
| Reviewed commit | `8b1e7129ce29dc60f1da0851ccedff2a874a7902` |
| Reviewed tree | `acd5769c92a0ab09693e235ce20660ada8e380c7` |
| Accepting human | Repository owner `progentic` (Proto), through the instruction in this task |
| Decision date | 2026-10-22, America/New_York |
| Decision | Accepted |
| Authority source | "accept phase 2 and start ADR-0004" |

The authenticated GitHub account was checked as `progentic` when recording the
instruction. The acceptance is bound to the reviewed implementation, not to an
unreviewed future revision.

## Accepted Scope

The [authority contract](../AUTHORITY_CONTRACT.md) implements the contract
decisions in [ADR 0002](../adr/0002-runtime-language.md) and
[ADR 0003](../adr/0003-authority-contract.md), including the subsequent input and
diagnostic hardening in the reviewed commit:

- Strict, versioned JSON rejects missing and unknown fields, duplicate keys,
  unsupported schema versions, malformed values, and unsupported capabilities.
- Exact relative paths and separate read, write, create, and protected-action
  capabilities preserve the human-defined policy ceiling.
- Requested scope is compared with the entire independently supplied approval.
  Reusing an approval reference cannot authorize changed scope.
- Policy-required invariants are retained; unknown invariant identifiers fail
  validation.
- Canonical policy and task snapshots are deterministic, independent of mutable
  inputs, and exposed only through shared borrows.
- Document, set, identity, and path limits bound validation inputs. Errors and
  Debug output omit supplied values.

## Verification Evidence

The [main-branch Ubuntu CI run](https://github.com/progentic/aegis/actions/runs/39885057064)
passed for the exact reviewed commit. It completed on 2026-10-23 UTC
(2026-10-22 in America/New_York).

| Check | Result at the reviewed commit |
| :--- | :--- |
| Rust representation tests | 3 passed |
| Rust contract-validation tests | 22 passed |
| Rust security-boundary tests | 21 passed |
| Rust compile-failure documentation tests | 4 passed |
| Python foundation tests | 32 passed |
| Rust formatting, Clippy with warnings denied, tests, and build | Passed on Ubuntu 24.04 CI |
| Foundation and local Markdown reference validation | Passed |
| Pinned workflow lint and unchanged-source check | Passed |

## Acceptance Limits

The caller must still establish the provenance and currency of policy and
approval inputs independently. Matching identity strings are not authentication.
Recognizing an invariant identifier does not run its enforcement check.

This acceptance covers the pure contract portion of Phase 2. It does not accept
a decision engine, executable permits, repository mediation, executor isolation,
provider access, a version tag, or production admission. Those retain their
[roadmap](../ROADMAP.md) and [security-boundary](../SECURITY_BOUNDARY.md) gates.

The instruction authorizes starting the
[ADR 0004 proposal](../adr/0004-decision-core.md). It does not accept that ADR or
authorize its implementation. The separate Phase 1 acceptance record is unchanged.
