#!/usr/bin/env python3
"""Validate deterministic Phase 1 repository foundation facts."""

from __future__ import annotations

import os
import re
import sys
from dataclasses import dataclass
from datetime import date
from html import unescape
from pathlib import Path
from typing import Sequence
from urllib.parse import unquote, urlsplit


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
AGENTS_PATH = Path("AGENTS.md")
BOOTSTRAP_DOCUMENTS = (
    Path("docs/ARCHITECTURE.md"),
    Path("docs/GOVERNANCE.md"),
    Path("docs/governance/AUTHORITY_GOVERNANCE.md"),
    Path("docs/governance/BOUNDARY_GOVERNANCE.md"),
    Path("docs/governance/ESCALATION_GOVERNANCE.md"),
    Path("docs/governance/FEATURE_GOVERNANCE.md"),
)
PHASE_ONE_DOCUMENTS = (
    Path("README.md"),
    Path("docs/acceptance/PHASE-1.md"),
    *(path for path in REQUIRED_PHASE_ONE_FILES if path.suffix == ".md"),
)
IGNORED_LINK_SCHEMES = frozenset(("http", "https", "mailto"))
FENCED_CODE_PATTERN = re.compile(
    r"(?ms)^ {0,3}(?P<fence>`{3,}|~{3,})[^\n]*\n.*?^ {0,3}(?P=fence)[ \t]*$"
)
INLINE_CODE_PATTERN = re.compile(r"(`+)(?:(?!\1).)*?\1", re.DOTALL)
INLINE_LINK_START_PATTERN = re.compile(r"!?\[(?:\\.|[^\]\\\n])*\]\(\s*")
LINK_TAIL_PATTERN = re.compile(
    r"""\s*(?:"[^"\n]*"|'[^'\n]*'|\([^()\n]*\))?\s*\)"""
)
REFERENCE_DEFINITION_PATTERN = re.compile(
    r"(?m)^ {0,3}\[[^\]\n]+\]:[ \t]*(<[^>\n]*>|[^\s]+)"
)
MARKDOWN_ESCAPE_PATTERN = re.compile(r"\\([^\w\s])")
MISSING_TARGET = "missing target"
WRONG_CASE_TARGET = "case-mismatched target"
OUTSIDE_REPOSITORY = "target outside repository"


@dataclass(frozen=True)
class MarkdownReference:
    source: Path
    original: str
    target: Path
    lexical: Path


class DocumentReferenceError(RuntimeError):
    """A documentation source or reference could not be read or resolved."""


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
        *validate_local_markdown_references(repository_root),
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


def validate_local_markdown_references(repository_root: Path) -> tuple[str, ...]:
    issues = []
    for source in PHASE_ONE_DOCUMENTS:
        issues.extend(validate_document_references(repository_root, source))
    return tuple(issues)


def validate_document_references(root: Path, source: Path) -> tuple[str, ...]:
    try:
        references = read_document_references(root, source)
    except DocumentReferenceError as error:
        return (str(error),)
    issues = [issue for ref in references for issue in validate_reference(root, ref)]
    if source == AGENTS_PATH:
        issues.extend(validate_bootstrap_routes(root, references))
    return tuple(issues)


def validate_reference(root: Path, reference: MarkdownReference) -> tuple[str, ...]:
    if not is_within_repository(root, reference.target):
        return (describe_reference(reference, OUTSIDE_REPOSITORY),)
    if not target_exists(reference.target):
        return (describe_reference(reference, MISSING_TARGET),)
    if not has_exact_case(root, reference.lexical):
        return (describe_reference(reference, WRONG_CASE_TARGET),)
    return ()


def validate_bootstrap_routes(
    root: Path,
    references: Sequence[MarkdownReference],
) -> tuple[str, ...]:
    issues = []
    for source in BOOTSTRAP_DOCUMENTS:
        target = resolve_document_path(root, source)
        if not any(reference.target == target for reference in references):
            issues.append(
                f"{AGENTS_PATH}: missing required bootstrap Markdown link to {source}"
            )
    return tuple(issues)


def read_document_references(root: Path, source: Path) -> tuple[MarkdownReference, ...]:
    try:
        source_path = resolve_document_path(root, source)
        if not is_within_repository(root, source_path):
            raise DocumentReferenceError(
                f"{source}: documentation source outside repository"
            )
        text = source_path.read_text(encoding="utf-8")
        return resolve_markdown_references(root, source, text)
    except (OSError, UnicodeError, ValueError) as error:
        raise DocumentReferenceError(
            f"{source}: could not check local Markdown references: {error}"
        ) from error


def resolve_markdown_references(
    root: Path,
    source: Path,
    text: str,
) -> tuple[MarkdownReference, ...]:
    references = []
    for original in extract_markdown_destinations(text):
        path = local_reference_path(original)
        if path is not None:
            joined = root / source.parent / path
            target = joined.resolve()
            lexical = Path(os.path.normpath(joined))
            references.append(MarkdownReference(source, original, target, lexical))
    return tuple(references)


def extract_markdown_destinations(text: str) -> tuple[str, ...]:
    prose = mask_markdown_code(text)
    inline = []
    for match in INLINE_LINK_START_PATTERN.finditer(prose):
        target, end = read_inline_destination(prose, match.end())
        if target and LINK_TAIL_PATTERN.match(prose, end):
            inline.append(target)
    definitions = REFERENCE_DEFINITION_PATTERN.findall(prose)
    return (*inline, *(target.strip("<>") for target in definitions))


def mask_markdown_code(text: str) -> str:
    without_fences = FENCED_CODE_PATTERN.sub("", text)
    return INLINE_CODE_PATTERN.sub("", without_fences)


def read_inline_destination(text: str, start: int) -> tuple[str, int]:
    if start < len(text) and text[start] == "<":
        end = text.find(">", start + 1)
        if end == -1:
            return "", start
        return text[start + 1:end], end + 1
    return read_bare_destination(text, start)


def read_bare_destination(text: str, start: int) -> tuple[str, int]:
    end, depth = start, 0
    while end < len(text):
        character = text[end]
        if character.isspace() or (character == ")" and depth == 0):
            break
        if character == "\\":
            end += 2
            continue
        depth += parenthesis_delta(character)
        end += 1
    return text[start:end], end


def parenthesis_delta(character: str) -> int:
    return int(character == "(") - int(character == ")")


def local_reference_path(original: str) -> str | None:
    decoded = unescape(MARKDOWN_ESCAPE_PATTERN.sub(r"\1", original))
    if decoded.partition(":")[0].lower() in IGNORED_LINK_SCHEMES:
        return None
    parts = urlsplit(decoded)
    if parts.scheme or parts.netloc:
        raise ValueError(f"unsupported reference URI: {original!r}")
    if not parts.path:
        return None
    return unquote(parts.path)


def resolve_document_path(root: Path, source: Path) -> Path:
    return (root / source).resolve()


def is_within_repository(root: Path, target: Path) -> bool:
    try:
        target.relative_to(root.resolve())
    except ValueError:
        return False
    return True


def target_exists(target: Path) -> bool:
    return target.is_file() or target.is_dir()


def has_exact_case(root: Path, lexical: Path) -> bool:
    # Case-insensitive filesystems accept a link such as governance.md for
    # GOVERNANCE.md, but Linux and GitHub do not. Compare each path component
    # with the real directory listing so the result does not depend on the
    # filesystem running the check.
    try:
        parts = lexical.relative_to(root).parts
    except ValueError:
        return True  # A target outside the repository is reported separately.
    current = root
    for part in parts:
        try:
            names = os.listdir(current)
        except OSError:
            return False
        if part not in names:
            return False
        current = current / part
    return True


def describe_reference(reference: MarkdownReference, reason: str) -> str:
    return (
        f"{reference.source}: local Markdown reference {reference.original!r} "
        f"resolves to {reason}: {reference.target}"
    )


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
