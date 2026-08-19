#!/usr/bin/env python3
"""Validate deterministic Phase 1 repository foundation facts."""

from __future__ import annotations

import re
import sys
from datetime import date
from pathlib import Path
from typing import Pattern, Sequence


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
UNRELEASED_HEADING = "## [Unreleased]"
VERSION_HEADING_PATTERN = re.compile(r"^## \[[^]]+\](?: - .+)?$", re.MULTILINE)
INITIAL_VERSION_PATTERN = re.compile(r"^## \[0\.0\.0\] - (\d{4}-\d{2}-\d{2})$")
MISSING_FILE_MESSAGE = "missing required Phase 1 file: "
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
    issue_groups = (
        validate_required_files(repository_root),
        validate_unreleased_section(repository_root),
        validate_initial_version(repository_root),
    )
    return combine_issue_groups(issue_groups)


def locate_repository_root() -> Path:
    return ascend_from_file(Path(__file__), parent_count=2)


def validate_required_files(repository_root: Path) -> tuple[str, ...]:
    missing_files = find_missing_files(repository_root, REQUIRED_PHASE_ONE_FILES)
    return format_path_issues(missing_files, MISSING_FILE_MESSAGE)


def validate_unreleased_section(repository_root: Path) -> tuple[str, ...]:
    changelog_text = read_optional_text(repository_root / CHANGELOG_PATH)
    if changelog_text is None:
        return ()
    if contains_exact_line(changelog_text, UNRELEASED_HEADING):
        return (UNRELEASED_MESSAGE,)
    return ()


def validate_initial_version(repository_root: Path) -> tuple[str, ...]:
    changelog_text = read_optional_text(repository_root / CHANGELOG_PATH)
    if changelog_text is None:
        return ()
    first_heading = find_first_match(changelog_text, VERSION_HEADING_PATTERN)
    if not is_valid_dated_heading(first_heading, INITIAL_VERSION_PATTERN):
        return (INITIAL_VERSION_MESSAGE,)
    return ()


def report_validation(issues: Sequence[str]) -> int:
    if issues:
        write_error_report(FAILURE_MESSAGE, issues)
        return FAILURE_EXIT_CODE
    write_output_line(SUCCESS_MESSAGE)
    return SUCCESS_EXIT_CODE


def combine_issue_groups(issue_groups: Sequence[Sequence[str]]) -> tuple[str, ...]:
    return tuple(issue for group in issue_groups for issue in group)


def ascend_from_file(file_path: Path, parent_count: int) -> Path:
    return file_path.resolve().parents[parent_count - 1]


def find_missing_files(
    root_path: Path,
    relative_paths: Sequence[Path],
) -> tuple[Path, ...]:
    return tuple(path for path in relative_paths if not (root_path / path).is_file())


def format_path_issues(paths: Sequence[Path], prefix: str) -> tuple[str, ...]:
    return tuple(f"{prefix}{path.as_posix()}" for path in paths)


def read_optional_text(file_path: Path) -> str | None:
    try:
        return file_path.read_text(encoding="utf-8")
    except FileNotFoundError:
        return None


def contains_exact_line(text: str, expected_line: str) -> bool:
    return expected_line in text.splitlines()


def find_first_match(text: str, pattern: Pattern[str]) -> str | None:
    match = pattern.search(text)
    return match.group(0) if match is not None else None


def is_valid_dated_heading(heading: str | None, pattern: Pattern[str]) -> bool:
    if heading is None:
        return False
    match = pattern.fullmatch(heading)
    if match is None:
        return False
    return is_iso_date(match.group(1))


def is_iso_date(value: str) -> bool:
    try:
        date.fromisoformat(value)
    except ValueError:
        return False
    return True


def write_error_report(heading: str, issues: Sequence[str]) -> None:
    write_error_line(heading)
    for issue in issues:
        write_error_line(f"- {issue}")


def write_output_line(message: str) -> None:
    print(message)


def write_error_line(message: str) -> None:
    print(message, file=sys.stderr)


if __name__ == "__main__":
    raise SystemExit(main())
