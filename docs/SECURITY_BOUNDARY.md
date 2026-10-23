# Security Boundary — Maintainer Reference

## Current Supported Surface

AEGIS currently exposes a pure Rust authority-contract library. It accepts
JSON declarations and compares a task with independently trusted policy and
approved scope. It does not accept credentials, execute commands, contact
providers, write repositories, issue permits, or implement a model-facing
service. The experimental Python drafts are not part of the shipped package.

Use `parse_policy` and `parse_task` for untrusted JSON. Direct Serde
deserialization and public input constructors produce unvalidated declarations.
`validate_contract` rechecks their field and set limits before producing an
immutable result. Neither path authenticates a human or creates execution rights.

The [contract reference](AUTHORITY_CONTRACT.md) defines the parser limits,
diagnostic codes, and authority comparisons. The
[security regression tests](../tests/security_boundary.rs) exercise oversized
inputs, credential and command injection, safe diagnostics, immutable snapshots,
and unsupported capabilities. `cargo test --locked --offline` runs these tests
and the compile-failure checks that prevent construction, deserialization, or
mutation of a validated result through its public interface.

## Credential Handling

No credential store or provider adapter is implemented. Secret, environment,
process, network, decision, and permit fields are rejected rather than ignored
or interpreted. A protected `ServiceConnection` grant concerns repository
changes; it does not authorize network execution or access to a secret.

Debug output redacts authority declarations and snapshots. Errors contain
stable codes rather than supplied values, filenames, parser excerpts, or
unknown field names. Explicit serialization and accessors still expose the
original data to the trusted caller; do not log raw documents or treat redacted
Debug output as an audit record. Git ignore rules help prevent accidental
credential commits but are not access controls or secret detection.

## Required Runtime Admission Evidence

These requirements follow the accepted
[deployment decision](adr/0001-deployment-profile-and-threat-model.md). They
remain unimplemented and must be tested before an executable gateway is admitted.

| Boundary | Required rejection or containment test |
| :--- | :--- |
| Human authority | Model-supplied identity strings, policy, approvals, decisions, or permits cannot establish trusted provenance. |
| Execution identity | The executor and its descendants cannot read control state or write the authoritative repository, enforcement code, permits, or evidence. |
| Filesystem mediation | Direct writes, traversal, rename, symlink, hard-link, permission, and descriptor-passing bypasses fail on Ubuntu 24.04 LTS. |
| Checked action | Different bytes, targets, operation types, repository identity, or relevant repository state invalidate authorization. |
| Permit lifetime | Forged, stale, expired, revoked, or replayed permits cannot execute. Revocation is checked at use, not only issuance. |
| Process environment | A sentinel credential in the mediator environment is absent from the executor and every child process. |
| Network and secrets | Unsupported destinations, cloud metadata endpoints, raw secret reads, and ungranted provider operations fail closed. |
| Required controls | Policy, parser, evidence, identity, or enforcement failures cannot lead to a repository change. Interrupted operations require attributable recovery. |

Build the trusted components outside the model-writable staging workspace,
using the committed dependency lockfile and toolchain. Record source, toolchain,
dependency, and artifact identity. The existing Linux workflow verifies the pure
contract; it does not establish the runtime isolation evidence in this table.

The deployment trusts the administrator, kernel, and shipped AEGIS components.
It does not claim protection against their compromise. Future provider access
must retain durable credentials outside model-controlled processes, constrain
the operation and resources, and provide independent issuer-side revocation.
Adding such access requires explicit authority and its own reviewed adapter;
this hardening change adds none.
