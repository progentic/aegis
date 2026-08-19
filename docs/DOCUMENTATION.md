# AEGIS Documentation Standard

## Purpose

Documentation explains what AEGIS protects, what each part does, and how to operate or maintain it safely.

Write for a reader who has not seen the project before. Do not assume the reader knows the architecture, security model, acronyms, or repository layout.

## Two Audiences

Every document serves one primary audience.

### User Documentation

User documentation helps a person operate AEGIS.

It should explain:

- what the operation does
- what input is required
- what will change
- what will not change
- what permission is needed
- what success looks like
- what common failures mean
- how to stop or recover safely

Do not require a user to understand internal classes, modules, or implementation details.

### Maintainer Documentation

Maintainer documentation helps a contributor understand and change AEGIS without weakening its authority boundary.

It should explain:

- the responsibility of the component
- the inputs and outputs
- the source of authority
- the invariants that apply
- failure behavior
- evidence produced
- protected boundaries
- relevant tests
- the ADRs that explain important decisions

Do not mix user instructions and maintainer internals in the same procedure. Link between them when both are needed.

## Language Rules

- Prefer plain language at about a 12th-grade reading level.
- Use short paragraphs. Most paragraphs should contain one to four sentences.
- Do not create walls of text.
- Define a technical term before using it repeatedly.
- Expand an acronym the first time it appears.
- Use one term for one concept. Do not rotate through synonyms.
- Use active voice when it makes the actor clear.
- State requirements with **must** and recommendations with **should**.
- Avoid hype, vague assurances, and claims that cannot be tested.
- Do not describe planned behavior as implemented behavior.

Bad:

> The mediation subsystem leverages deterministic policy primitives to facilitate robust governance over heterogeneous repository mutation vectors.

Good:

> The Repository Mediator is the only path that may change the repository. It applies a change only after the Decision Engine confirms that the action is authorized.

## Define the Concrete Object First

Start with the object or action the reader can see.

Good order:

1. Name the object.
2. State what it protects or connects.
3. Explain the problem it prevents.
4. Define the technical term.
5. Add internal detail only when needed.

Example:

> A task scope is the list of actions approved for one task. It names the files the model may read, write, or create. This prevents the model from treating broad repository access as implied permission.

## Structure

Use headings that help a reader find an answer quickly.

A procedure should normally contain:

1. purpose
2. prerequisites
3. steps
4. expected result
5. failure and recovery guidance

A maintainer reference should normally contain:

1. responsibility
2. boundary
3. inputs and outputs
4. invariants
5. failure behavior
6. tests
7. related ADRs and documents

Use:

- bullets for sets of items
- numbered lists for ordered actions
- tables for exact comparisons or mappings
- code blocks for commands, configuration, and literal output
- diagrams only when relationships are easier to understand visually

## Commands and Examples

Examples must be safe to copy.

- Show the working directory when it matters.
- Use placeholders that are clearly marked.
- Quote paths and variables correctly.
- Do not include real secrets, tokens, or personal data.
- State whether a command reads, writes, or deletes data.
- Include expected output when the result is not obvious.
- Mark destructive commands and provide a safer recovery path.

## Accuracy and Status

Documentation must distinguish among:

- **Implemented:** present and verified in the repository
- **Planned:** approved direction that is not complete
- **Proposed:** under consideration and not approved
- **Deprecated:** still present but scheduled for removal

Do not use “secure,” “safe,” “production-ready,” or “complete” without naming the evidence and limits behind the claim.

## Decision Records

Use an Architecture Decision Record (ADR) when a decision changes a durable system boundary, authority source, invariant, dependency direction, evidence contract, or failure model.

An ADR should state:

- the problem
- the decision
- the alternatives considered
- the reason for the choice
- the consequences
- affected invariants and phases

The phase map may change as ADRs refine implementation order. The roadmap changes only through its human-controlled process.

The canonical ADR index is [`docs/adr/README.md`](adr/README.md). Start a new record from [`docs/adr/0000-template.md`](adr/0000-template.md).

## Required Updates

Update documentation in the same change when work alters:

- user-visible behavior
- configuration or command syntax
- required permissions
- failure or recovery behavior
- an invariant
- an authority or protected boundary
- evidence fields
- a public interface
- a roadmap or phase gate, when explicitly authorized

If no documentation change is needed, the change description should say why.

## Review Checklist

Before merging documentation:

- identify the primary audience
- verify every command and path
- define new terms and acronyms
- break up long paragraphs
- remove repeated explanations
- separate current behavior from planned behavior
- link the governing document or ADR
- confirm that lower-level text does not weaken governance
- check that the document says what the system cannot do
