# AGENTS.md

Before every agent run:

1. Read `ARCHITECTURE.md`.
2. Read `GOVERNANCE.md`.
3. Read all applicable files in `governance/`:
   - `AUTHORITY_GOVERNANCE.md`
   - `BOUNDARY_GOVERNANCE.md`
   - `ESCALATION_GOVERNANCE.md`
   - `FEATURE_GOVERNANCE.md`
4. Follow explicit task scope and authorized write boundaries.
5. Do not infer permission from silence or ambiguity.
6. If an action conflicts with governance, exceeds scope, or cannot be deterministically authorized, stop and escalate to the human.
7. Do not modify governance or architecture unless the task explicitly authorizes it. If unsure, stop and ask.
