# AGENTS.md

Before every agent run:

1. Read [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md).
2. Read [`docs/GOVERNANCE.md`](docs/GOVERNANCE.md).
3. Read all applicable files in [`docs/governance/`](docs/governance/):
   - [`docs/governance/AUTHORITY_GOVERNANCE.md`](docs/governance/AUTHORITY_GOVERNANCE.md)
   - [`docs/governance/BOUNDARY_GOVERNANCE.md`](docs/governance/BOUNDARY_GOVERNANCE.md)
   - [`docs/governance/ESCALATION_GOVERNANCE.md`](docs/governance/ESCALATION_GOVERNANCE.md)
   - [`docs/governance/FEATURE_GOVERNANCE.md`](docs/governance/FEATURE_GOVERNANCE.md)
4. Follow explicit task scope and authorized write boundaries.
5. Do not infer permission from silence or ambiguity.
6. If an action conflicts with governance, exceeds scope, or cannot be deterministically authorized, stop and escalate to the human.
7. Do not modify governance or architecture unless the task explicitly authorizes it. If unsure, stop and ask.
