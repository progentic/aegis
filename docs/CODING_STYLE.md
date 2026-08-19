# AEGIS Coding Style

## Rule

Write simple code that is easy to inspect.

Prefer explicit steps over compact tricks. A maintainer should understand the control flow without decoding it.

## General Rules

- Use the smallest clear solution.
- Give functions and variables specific names.
- Keep functions focused on one job.
- Make inputs, outputs, and failure states visible.
- Validate data at system boundaries.
- Return errors with useful context.
- Avoid hidden mutation and global state.
- Avoid metaprogramming, dynamic evaluation, and clever one-liners.
- Do not catch an error unless the code can handle it or add useful context.
- Add a dependency only when task scope explicitly permits it.
- Comments explain why a constraint exists. Code should show what it does.

Security-sensitive code should be boring. Boring code is easier to review and test.

## Python

Use type hints for public functions. Prefer small functions, direct branches, standard-library tools, and specific exceptions.

### Conditions

Bad:

```python
decision = "ALLOW" if authorized and in_scope and not violation else "STOP"
```

Good:

```python
def decide_action(
    authorized: bool,
    in_scope: bool,
    violates_invariant: bool,
) -> str:
    if not authorized:
        return "STOP"

    if not in_scope:
        return "ESCALATE"

    if violates_invariant:
        return "STOP"

    return "ALLOW"
```

The good example preserves the reason for each result.

### File Handling

Bad:

```python
data = open(path).read()
```

Good:

```python
from pathlib import Path


def read_policy(policy_path: Path) -> str:
    try:
        return policy_path.read_text(encoding="utf-8")
    except OSError as error:
        raise RuntimeError(f"could not read policy: {policy_path}") from error
```

The good example closes the file, sets the text encoding, and preserves the original error.

### Exceptions

Bad:

```python
try:
    apply_change(change)
except Exception:
    pass
```

Good:

```python
try:
    apply_change(change)
except PermissionError as error:
    raise ChangeDenied("repository write was not authorized") from error
```

Never hide a failure in authority or repository code.

## Bash

Use Bash only for short automation and command orchestration. Move complex parsing or decision logic into a language with safer data handling.

Start scripts with strict error handling:

```bash
#!/usr/bin/env bash
set -Eeuo pipefail
```

Quote variable expansions. Use arrays for command arguments. Validate every path before a destructive or repository-affecting action.

### Variables

Bad:

```bash
for file in $FILES; do
  check $file
done
```

Good:

```bash
files=("docs/INVARIANTS.md" "GOVERNANCE.md")

for file in "${files[@]}"; do
  check_file "$file"
done
```

The good example preserves spaces and argument boundaries.

### Required Inputs

Bad:

```bash
target=$1
run_check $target
```

Good:

```bash
if (( $# != 1 )); then
  echo "usage: $0 <target-path>" >&2
  exit 2
fi

target_path=$1
run_check "$target_path"
```

The good example rejects missing and extra arguments.

### Destructive Commands

Bad:

```bash
rm -rf "$OUTPUT_DIR"
```

Good:

```bash
output_dir=$1

if [[ -z "$output_dir" || "$output_dir" == "/" ]]; then
  echo "refusing unsafe output directory" >&2
  exit 1
fi

if [[ "$output_dir" != "$PWD/build/"* ]]; then
  echo "output directory is outside the build boundary" >&2
  exit 1
fi

rm -rf -- "$output_dir"
```

Prefer a recoverable operation when one is available.

### Dynamic Execution

Bad:

```bash
eval "$COMMAND"
```

Good:

```bash
command_args=(python3 scripts/validate_policy.py --policy "$policy_path")
"${command_args[@]}"
```

Do not use `eval` to build commands from text.

## Required Checks

Before merging:

- run the language formatter
- run the language linter
- run static checks
- run relevant tests
- run `shellcheck` on Bash scripts
- confirm errors are not ignored
- confirm the change stays within authorized scope

Tool output supports review. Passing checks does not grant repository authority.
