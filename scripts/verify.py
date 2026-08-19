#!/usr/bin/env python3
"""Validate deterministic Phase 1 repository foundation facts."""

from __future__ import annotations

import re
import sys
from datetime import date
from pathlib import Path
from typing import Sequence


REQUIRED_PHASE_ONE_FILES = (
    Path("AGENTS.md"),
    Path("CHANGELOG.md"),
    Path("docs/ARCHITECTURE.md"),
    Path("docs/CODING_STYLE.md"),
    Path("docs/DOCUMENTATION.md"),
    Path("docs/GOVERNANCE.md"),
    Path("docs/INVARIANTS.md"),
    Path("docs/PHASEMAP.md"),
    Path("docs/ROADMAP.md"),
    Path("docs/TASKS.md"),
    Path("docs/adr/0000-template.md"),
    Path("docs/adr/README.md"),
    Path("docs/governance/AUTHORITY_GOVERNANCE.md"),
    Path("docs/governance/BOUNDARY_GOVERNANCE.md"),
    Path("docs/governance/ESCALATION_GOVERNANCE.md"),
    Path("docs/governance/FEATURE_GOVERNANCE.md"),
    Path("scripts/verify.py"),
    Path("tests/test_verify.py"),
)
CHANGELOG_PATH = Path("CHANGELOG.md")
REQUIRED_CHANGELOG_PREAMBLE = (
    "# Changelog\n\n"
    "All notable changes to this project will be documented in this file.\n\n"
    "The format is based on Keep a Changelog, and this project adheres to "
    "Semantic Versioning.\n"
)
UNRELEASED_HEADING_PATTERN = re.compile(
    r"^[ \t]{0,3}##[ \t]+\[unreleased\](?:[ \t]+-.*)?[ \t]*$",
    re.IGNORECASE | re.MULTILINE,
)
VERSION_HEADING_PATTERN = re.compile(r"^## \[[^]]+\](?: - .+)?$", re.MULTILINE)
INITIAL_VERSION_PATTERN = re.compile(r"^## \[0\.0\.0\] - (\d{4}-\d{2}-\d{2})$")
MISSING_FILE_MESSAGE = "missing required Phase 1 file: "
PREAMBLE_MESSAGE = (
    "CHANGELOG.md is missing the required Keep a Changelog preamble or the "
    "preamble has changed"
)
UNRELEASED_MESSAGE = "CHANGELOG.md must not contain an Unreleased section"
INITIAL_VERSION_MESSAGE = "the first changelog version must be 0.0.0 with a valid date"
SUCCESS_MESSAGE = "Phase 1 repository foundation validation passed."
FAILURE_MESSAGE = "Phase 1 repository foundation validation failed:"
SUCCESS_EXIT_CODE = 0
FAILURE_EXIT_CODE = 1


def main() -> int:
    repository_root = locate_repository_root()
    issues = validate_foundation(repository_root)
    return report_validation(issues)


def validate_foundation(repository_root: Path) -> tuple[str, ...]:
    changelog_text = read_changelog(repository_root)
    return (
        *validate_required_files(repository_root),
        *validate_changelog_preamble(changelog_text),
        *validate_unreleased_section(changelog_text),
        *validate_initial_version(changelog_text),
    )


def locate_repository_root() -> Path:
    return Path(__file__).resolve().parents[1]


def validate_required_files(repository_root: Path) -> tuple[str, ...]:
    issues = []
    for relative_path in REQUIRED_PHASE_ONE_FILES:
        if not (repository_root / relative_path).is_file():
            issues.append(f"{MISSING_FILE_MESSAGE}{relative_path.as_posix()}")
    return tuple(issues)


def read_changelog(repository_root: Path) -> str | None:
    try:
        return (repository_root / CHANGELOG_PATH).read_text(encoding="utf-8")
    except FileNotFoundError:
        return None


def validate_changelog_preamble(changelog_text: str | None) -> tuple[str, ...]:
    if changelog_text is None:
        return ()
    if not changelog_text.startswith(REQUIRED_CHANGELOG_PREAMBLE):
        return (PREAMBLE_MESSAGE,)
    return ()


def validate_unreleased_section(changelog_text: str | None) -> tuple[str, ...]:
    if changelog_text is None:
        return ()
    if UNRELEASED_HEADING_PATTERN.search(changelog_text):
        return (UNRELEASED_MESSAGE,)
    return ()


def validate_initial_version(changelog_text: str | None) -> tuple[str, ...]:
    if changelog_text is None:
        return ()
    first_heading_match = VERSION_HEADING_PATTERN.search(changelog_text)
    first_heading = first_heading_match.group(0) if first_heading_match else None
    if not is_valid_initial_version(first_heading):
        return (INITIAL_VERSION_MESSAGE,)
    return ()


def is_valid_initial_version(heading: str | None) -> bool:
    if heading is None:
        return False
    match = INITIAL_VERSION_PATTERN.fullmatch(heading)
    if match is None:
        return False
    try:
        date.fromisoformat(match.group(1))
    except ValueError:
        return False
    return True


def report_validation(issues: Sequence[str]) -> int:
    if issues:
        print(FAILURE_MESSAGE, file=sys.stderr)
        for issue in issues:
            print(f"- {issue}", file=sys.stderr)
        return FAILURE_EXIT_CODE
    print(SUCCESS_MESSAGE)
    return SUCCESS_EXIT_CODE


if __name__ == "__main__":
    raise SystemExit(main())
