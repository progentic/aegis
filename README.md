# AEGIS

AEGIS exists because an LLM can produce code that is syntactically valid, compiles, and passes tests while still being outside the requested task, inconsistent with repository invariants, or based on an invented implementation direction. AEGIS reduces that risk by controlling what the model is permitted to do before repository state is changed.

AEGIS does not decide how software should be built. It enforces authority that humans have already defined.
