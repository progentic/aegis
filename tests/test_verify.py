from __future__ import annotations

import tempfile
import unittest
from pathlib import Path

from scripts import verify


VALID_CHANGELOG = """# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog, and this project adheres to Semantic Versioning.

## [0.0.0] - 2026-08-18

### Added

- Established the test fixture.
"""
UNRELEASED_HEADINGS = (
    "## [Unreleased]",
    "## [Unreleased]   ",
    "## [unreleased]",
    "## [Unreleased] - Pending",
)
VALID_AGENTS_DOCUMENT = """# AGENTS.md

1. Read [Architecture](docs/ARCHITECTURE.md).
2. Read [Governance](docs/GOVERNANCE.md).
3. Read the applicable documents:
   - [Authority](docs/governance/AUTHORITY_GOVERNANCE.md)
   - [Boundary](docs/governance/BOUNDARY_GOVERNANCE.md)
   - [Escalation](docs/governance/ESCALATION_GOVERNANCE.md)
   - [Feature](docs/governance/FEATURE_GOVERNANCE.md)
"""
FORMER_AGENTS_DOCUMENT = """# AGENTS.md

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
"""
FORMER_AGENT_ROUTES = (
    "ARCHITECTURE.md",
    "GOVERNANCE.md",
    "governance/AUTHORITY_GOVERNANCE.md",
    "governance/BOUNDARY_GOVERNANCE.md",
    "governance/ESCALATION_GOVERNANCE.md",
    "governance/FEATURE_GOVERNANCE.md",
)
REFERENCE_SOURCE = Path("docs/INVARIANTS.md")


class FoundationValidationTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary_directory, self.repository_root = create_temporary_repository()

    def tearDown(self) -> None:
        dispose_temporary_repository(self.temporary_directory)

    def test_passes_when_required_files_are_present(self) -> None:
        self.assertEqual((), verify.validate_foundation(self.repository_root))

    def test_fails_when_required_file_is_missing(self) -> None:
        remove_relative_file(self.repository_root, Path("docs/ROADMAP.md"))

        issues = verify.validate_foundation(self.repository_root)

        self.assertIn(
            "missing required Phase 1 file: docs/ROADMAP.md",
            issues,
        )

    def test_fails_when_changelog_preamble_is_missing(self) -> None:
        write_changelog(self.repository_root, changelog_without_preamble())

        issues = verify.validate_foundation(self.repository_root)

        self.assertIn(verify.PREAMBLE_MESSAGE, issues)

    def test_fails_when_changelog_preamble_is_changed(self) -> None:
        write_changelog(self.repository_root, changelog_with_changed_preamble())

        issues = verify.validate_foundation(self.repository_root)

        self.assertIn(verify.PREAMBLE_MESSAGE, issues)

    def test_fails_for_unreleased_heading_variants_after_zero(self) -> None:
        for heading in UNRELEASED_HEADINGS:
            with self.subTest(heading=heading):
                write_changelog(self.repository_root, append_to_changelog(heading))

                issues = verify.validate_foundation(self.repository_root)

                self.assertIn(verify.UNRELEASED_MESSAGE, issues)

    def test_passes_when_unreleased_appears_only_in_prose(self) -> None:
        changelog = append_to_changelog(
            "This prose mentions unreleased work without creating a section."
        )
        write_changelog(self.repository_root, changelog)

        self.assertEqual((), verify.validate_foundation(self.repository_root))

    def test_fails_when_first_version_is_not_zero(self) -> None:
        write_changelog(self.repository_root, changelog_with_later_version())

        issues = verify.validate_foundation(self.repository_root)

        self.assertIn(verify.INITIAL_VERSION_MESSAGE, issues)

    def test_passes_with_zero_version_and_valid_date(self) -> None:
        write_changelog(self.repository_root, VALID_CHANGELOG)

        self.assertEqual((), verify.validate_foundation(self.repository_root))

    def test_fails_when_initial_version_date_is_invalid(self) -> None:
        write_changelog(self.repository_root, changelog_with_invalid_date())

        issues = verify.validate_foundation(self.repository_root)

        self.assertIn(verify.INITIAL_VERSION_MESSAGE, issues)

    def test_validation_does_not_modify_checked_files(self) -> None:
        before_validation = snapshot_required_files(self.repository_root)

        verify.validate_foundation(self.repository_root)

        after_validation = snapshot_required_files(self.repository_root)
        self.assertEqual(before_validation, after_validation)


class MarkdownReferenceValidationTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary_directory, self.repository_root = create_temporary_repository()

    def tearDown(self) -> None:
        dispose_temporary_repository(self.temporary_directory)

    def test_corrected_candidate_document_set_passes(self) -> None:
        self.assertEqual((), verify.validate_foundation(verify.locate_repository_root()))

    def test_missing_relative_target_reports_source_reference_and_target(self) -> None:
        write_reference_document(self.repository_root, "[Missing](missing.md)")
        issues = verify.validate_foundation(self.repository_root)
        expected_target = (self.repository_root / "docs/missing.md").resolve()
        self.assertIn(
            f"{REFERENCE_SOURCE}: local Markdown reference 'missing.md' "
            f"resolves to missing target: {expected_target}",
            issues,
        )

    def test_parent_directory_and_encoded_escapes_fail(self) -> None:
        for reference in (
            "../../outside.md",
            "../../outside.md?x=1#anchor",
            "%2E%2E/%2E%2E/outside.md",
        ):
            with self.subTest(reference=reference):
                write_reference_document(self.repository_root, f"[Escape]({reference})")
                issues = verify.validate_foundation(self.repository_root)
                self.assertTrue(
                    any("target outside repository" in issue for issue in issues)
                )

    def test_external_urls_are_ignored(self) -> None:
        for reference in (
            "http://invalid.example/missing",
            "https://invalid.example/missing",
            "mailto:test@example.invalid",
            "HTTPS://invalid.example/missing",
            "https://[invalid/path",
        ):
            with self.subTest(reference=reference):
                write_reference_document(self.repository_root, f"[External]({reference})")
                self.assertEqual((), verify.validate_foundation(self.repository_root))

    def test_unsupported_reference_uris_do_not_pass(self) -> None:
        for reference in (
            "file:///etc/passwd",
            "ftp://invalid.example/file",
            "//invalid.example/file",
        ):
            with self.subTest(reference=reference):
                write_reference_document(
                    self.repository_root, f"[Unsupported]({reference})"
                )
                issues = verify.validate_foundation(self.repository_root)
                self.assertTrue(
                    any("unsupported reference URI" in issue for issue in issues)
                )

    def test_same_document_anchors_are_ignored(self) -> None:
        write_reference_document(self.repository_root, "[Anchor](#absent-section)")
        self.assertEqual((), verify.validate_foundation(self.repository_root))

    def test_queries_fragments_and_parent_links_resolve(self) -> None:
        for reference in (
            "GOVERNANCE.md?view=raw#absent-section",
            "GOVERNANCE.md#absent-section",
            "../README.md?raw=1",
            "governance/?sort=name#anchor",
        ):
            with self.subTest(reference=reference):
                write_reference_document(self.repository_root, f"[Valid]({reference})")
                self.assertEqual((), verify.validate_foundation(self.repository_root))

    def test_exact_former_invariant_links_fail(self) -> None:
        for reference in ("../GOVERNANCE.md", "../ARCHITECTURE.md"):
            with self.subTest(reference=reference):
                write_reference_document(
                    self.repository_root, f"[`Document`]({reference})"
                )
                issues = verify.validate_foundation(self.repository_root)
                self.assertTrue(
                    any(
                        f"reference {reference!r} resolves to missing target" in issue
                        for issue in issues
                    )
                )

    def test_exact_former_agents_document_fails(self) -> None:
        write_relative_file(
            self.repository_root, Path("AGENTS.md"), FORMER_AGENTS_DOCUMENT
        )
        issues = verify.validate_foundation(self.repository_root)
        self.assertEqual(
            6,
            sum("missing required bootstrap Markdown link" in issue for issue in issues),
        )

    def test_exact_former_agents_routes_as_links_fail(self) -> None:
        for reference in FORMER_AGENT_ROUTES:
            with self.subTest(reference=reference):
                text = f"{VALID_AGENTS_DOCUMENT}\n[Former]({reference})\n"
                write_relative_file(self.repository_root, Path("AGENTS.md"), text)
                issues = verify.validate_foundation(self.repository_root)
                self.assertTrue(
                    any(
                        f"reference {reference!r} resolves to missing target" in issue
                        for issue in issues
                    )
                )

    def test_existing_target_outside_repository_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as outside_directory:
            outside_target = Path(outside_directory) / "outside.md"
            write_relative_file(
                Path(outside_directory), Path("outside.md"), "outside\n"
            )
            write_reference_document(self.repository_root, f"[Escape]({outside_target})")
            issues = verify.validate_foundation(self.repository_root)
            self.assertTrue(
                any("target outside repository" in issue for issue in issues)
            )

    def test_symlink_target_outside_repository_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as outside_directory:
            write_relative_file(
                Path(outside_directory), Path("outside.md"), "outside\n"
            )
            create_file_symlink(
                self.repository_root / "docs/linked.md",
                Path(outside_directory) / "outside.md",
            )
            write_reference_document(self.repository_root, "[Escape](linked.md)")
            issues = verify.validate_foundation(self.repository_root)
            self.assertTrue(
                any("target outside repository" in issue for issue in issues)
            )

    def test_inline_images_and_reference_definitions_are_checked(self) -> None:
        for text in (
            "![Image](missing.png)",
            "[Link][missing]\n\n[missing]: missing.md",
            "[Link][missing]\n\n[missing]: <missing file.md>",
        ):
            with self.subTest(text=text):
                write_reference_document(self.repository_root, text)
                issues = verify.validate_foundation(self.repository_root)
                self.assertTrue(any("missing target" in issue for issue in issues))

    def test_encoded_spaces_titles_and_balanced_parentheses_resolve(self) -> None:
        write_relative_file(
            self.repository_root, Path("docs/local (nested(copy)).md"), "target\n"
        )
        for reference in (
            '<local (nested(copy)).md> "title"',
            "local%20(nested(copy)).md",
            'GOVERNANCE.md "title"',
        ):
            with self.subTest(reference=reference):
                write_reference_document(self.repository_root, f"[Valid]({reference})")
                self.assertEqual((), verify.validate_foundation(self.repository_root))

    def test_code_examples_are_not_documentation_links(self) -> None:
        for text in (
            "`[Missing](missing.md)`",
            "```markdown\n[Missing](missing.md)\n```",
            "~~~markdown\n[Missing](missing.md)\n~~~",
        ):
            with self.subTest(text=text):
                write_reference_document(self.repository_root, text)
                self.assertEqual((), verify.validate_foundation(self.repository_root))

    def test_link_validation_is_deterministic_and_read_only(self) -> None:
        write_reference_document(
            self.repository_root,
            "[Valid](GOVERNANCE.md?raw=1#anchor)\n[Missing](missing.md)",
        )
        before = snapshot_required_files(self.repository_root)
        first = verify.validate_foundation(self.repository_root)
        second = verify.validate_foundation(self.repository_root)
        self.assertEqual(first, second)
        self.assertEqual(before, snapshot_required_files(self.repository_root))


def create_temporary_repository() -> tuple[tempfile.TemporaryDirectory, Path]:
    temporary_directory = tempfile.TemporaryDirectory()
    repository_root = Path(temporary_directory.name)
    create_repository_fixture(repository_root)
    return temporary_directory, repository_root


def dispose_temporary_repository(
    temporary_directory: tempfile.TemporaryDirectory,
) -> None:
    temporary_directory.cleanup()


def create_repository_fixture(repository_root: Path) -> None:
    for relative_path in (*verify.REQUIRED_PHASE_ONE_FILES, *verify.PHASE_ONE_DOCUMENTS):
        write_relative_file(repository_root, relative_path, "fixture\n")
    write_relative_file(repository_root, Path("AGENTS.md"), VALID_AGENTS_DOCUMENT)
    write_changelog(repository_root, VALID_CHANGELOG)


def remove_relative_file(repository_root: Path, relative_path: Path) -> None:
    (repository_root / relative_path).unlink()


def write_changelog(repository_root: Path, content: str) -> None:
    write_relative_file(repository_root, verify.CHANGELOG_PATH, content)


def changelog_without_preamble() -> str:
    return VALID_CHANGELOG.replace("# Changelog\n\n", "", 1)


def changelog_with_changed_preamble() -> str:
    return VALID_CHANGELOG.replace("Keep a Changelog", "a custom format", 1)


def append_to_changelog(content: str) -> str:
    return f"{VALID_CHANGELOG}\n{content}\n"


def changelog_with_later_version() -> str:
    return VALID_CHANGELOG.replace("[0.0.0]", "[0.1.0]")


def changelog_with_invalid_date() -> str:
    return VALID_CHANGELOG.replace("2026-08-18", "2026-13-40")


def write_relative_file(
    repository_root: Path,
    relative_path: Path,
    content: str,
) -> None:
    file_path = repository_root / relative_path
    file_path.parent.mkdir(parents=True, exist_ok=True)
    file_path.write_text(content, encoding="utf-8")


def write_reference_document(repository_root: Path, text: str) -> None:
    write_relative_file(repository_root, REFERENCE_SOURCE, text)


def create_file_symlink(link: Path, target: Path) -> None:
    link.symlink_to(target)


def snapshot_required_files(repository_root: Path) -> dict[Path, tuple[bytes, int]]:
    return {
        relative_path: read_file_snapshot(repository_root / relative_path)
        for relative_path in (*verify.REQUIRED_PHASE_ONE_FILES, *verify.PHASE_ONE_DOCUMENTS)
    }


def read_file_snapshot(file_path: Path) -> tuple[bytes, int]:
    return file_path.read_bytes(), file_path.stat().st_mtime_ns


if __name__ == "__main__":
    unittest.main()
