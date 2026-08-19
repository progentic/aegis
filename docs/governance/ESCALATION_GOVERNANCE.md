# Escalation Governance

## Purpose

This document defines when AEGIS allows, denies, stops, or escalates an LLM action.

Escalation is the mechanism that returns authority to a human when existing authorization is insufficient.

## Default Rule

Permission is affirmative, not inferred.

If AEGIS cannot deterministically establish that an action is authorized, it must not permit the action.

## Decision States

AEGIS uses four authority outcomes.

### ALLOW

Use only when AEGIS can deterministically establish that:

- the action is explicitly authorized
- the action is within task scope
- the action does not violate a known invariant
- any boundary expansion is already explicitly authorized
- sufficient deterministic evidence exists

### DENY

Use when the action is explicitly prohibited and policy does not permit elevation for that action.

Examples may include:

- model self-modification of protected governance policy
- access to explicitly forbidden secrets
- actions outside an immutable security boundary

### STOP

Use when execution must halt before repository state is affected.

STOP may be used for a clear unauthorized action that can be corrected within existing authority, such as attempting to write an unapproved file.

### ESCALATE

Use when the requested action may be legitimate but exceeds current authority or cannot be deterministically resolved.

Escalation returns the decision to a human.

## Mandatory Escalation Conditions

AEGIS must escalate when:

- task scope is ambiguous
- applicable policy is incomplete or conflicting
- deterministic evidence is insufficient
- a protected boundary expansion is detected without authorization
- a requested capability exceeds the current lease
- a schema, permission, dependency, public interface, service connection, or protected-path change requires human approval
- a repository-specific policy explicitly requires human elevation

## No Probabilistic Fallback

AEGIS must not resolve an uncertain authorization decision through:

- model confidence
- embeddings
- similarity scores
- LLM judges
- heuristic assessments of whether the change is "probably safe"
- inferred human intent
- code-quality scores
- test success

These mechanisms may assist humans or other tools, but they cannot convert uncertainty into authority.

## Elevation Request

An elevation request should present facts necessary for a human to decide.

The model may provide an explanation, but AEGIS must independently provide the authority-relevant facts it can observe.

A useful elevation record includes:

```text
Task ID
Current authorization
Requested action
Boundary event type
Files affected
Protected paths affected
Manifest/schema/policy changes detected
Current capability or lease
Requested additional capability
Model-provided reason
Alternatives reported by model
Policy version
```

The human approves or rejects the requested authority.

## Human Approval Scope

Approval must be specific.

A human elevation should identify:

- the action being authorized
- the files or resources covered
- the capability granted
- applicable limits
- expiration or task lifetime where appropriate

Approval for one action must not become general authorization for unrelated actions.

## Evidence Requirement

AEGIS records why an action was allowed, denied, stopped, or escalated.

The record should distinguish:

- human-provided authority
- deterministic repository facts
- model-provided explanation

Model explanation must never be recorded as the authority source unless a human explicitly converts it into an approval.

## Fail Closed

If AEGIS itself cannot safely evaluate authority because of missing policy, parser failure, unavailable required evidence, corrupted state, or an enforcement error, it fails closed for repository-affecting actions.

Fail closed means the repository is not modified until authority can be re-established.
