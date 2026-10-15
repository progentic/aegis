# Phase 1 Acceptance Record

## Purpose

This record shows what a named human reviewed and accepted for Phase 1 (the `v0.0.0` foundation). Passing tests do not accept Phase 1. Only the accepting person's entry below does.

This record does not claim that any runtime control is implemented. See [`ROADMAP.md`](../ROADMAP.md) for later phases and [`INVARIANTS.md`](../INVARIANTS.md) for the rules those phases must satisfy.

## Status

**Pending acceptance.** No person has accepted Phase 1.

## Reviewed Source

Fill in these fields from the commit being reviewed. Do not reuse them for a later commit.

| Field | Value |
| :--- | :--- |
| Reviewed commit | `d24271eb380029545c631d4a7cc21709c59722a1` |
| Reviewed tree | `eac07eb9b25d16f0449722dbe7f935fb64a27f65` |
| Checklist revision | The revision of this file at the reviewed commit |

If the acceptance is for a later commit, replace both values and re-run every check below.

## Evidence

| Check | Environment | Date | Result |
| :--- | :--- | :--- | :--- |
| `python3 -m unittest discover -s tests -p 'test_verify.py'` | macOS, local run by the maintainer | 2026-10-14 | 26 tests passed (before the case-check tests) |
| Same command | Linux, Python 3.12.3, local run | 2026-10-14 | 26 tests passed (before the case-check tests) |
| Same command with this record's change applied | GitHub Actions, Linux | Pending | Pending: record the workflow run URL and tested source SHA |

A local run is not CI evidence. The CI row stays pending until the workflow has run on the exact commit being accepted.

## Checklist

The accepting person marks each item against the reviewed commit.

- [ ] A clean clone of the reviewed commit passes `python3 -m unittest discover -s tests -p 'test_verify.py'`.
- [ ] A clean clone passes `python3 scripts/verify.py`.
- [ ] The Linux CI run names the reviewed source SHA and passed.
- [ ] All local Markdown references resolve with exact letter case.
- [ ] `AGENTS.md` routes to the architecture, governance, and four governance documents under `docs/`.
- [ ] Governance and invariant meaning is unchanged from the reviewed foundation except for corrected links.
- [ ] The README states that AEGIS is at the foundation stage.
- [ ] The Limits in [`TASKS.md`](../TASKS.md) match what the validator actually checks.

## Decision

| Field | Value |
| :--- | :--- |
| Accepting person | Not yet recorded |
| Decision date | Not yet recorded |
| Decision | Not yet recorded (accepted, accepted with conditions, or rejected) |
| Conditions or limits | Not yet recorded |

Add the accepting person's name, the decision, and any conditions here in a separate commit. Do not edit the reviewed-source fields to match a different commit.
