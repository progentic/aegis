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

    def test_fails_when_changelog_contains_unreleased_section(self) -> None:
        write_changelog(self.repository_root, changelog_with_unreleased_section())

        issues = verify.validate_foundation(self.repository_root)

        self.assertIn(verify.UNRELEASED_MESSAGE, issues)

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
    for relative_path in verify.REQUIRED_PHASE_ONE_FILES:
        write_relative_file(repository_root, relative_path, "fixture\n")
    write_changelog(repository_root, VALID_CHANGELOG)


def remove_relative_file(repository_root: Path, relative_path: Path) -> None:
    (repository_root / relative_path).unlink()


def write_changelog(repository_root: Path, content: str) -> None:
    write_relative_file(repository_root, verify.CHANGELOG_PATH, content)


def changelog_with_unreleased_section() -> str:
    return VALID_CHANGELOG.replace(
        "## [0.0.0]",
        "## [Unreleased]\n\n## [0.0.0]",
    )


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


def snapshot_required_files(repository_root: Path) -> dict[Path, tuple[bytes, int]]:
    return {
        relative_path: read_file_snapshot(repository_root / relative_path)
        for relative_path in verify.REQUIRED_PHASE_ONE_FILES
    }


def read_file_snapshot(file_path: Path) -> tuple[bytes, int]:
    return file_path.read_bytes(), file_path.stat().st_mtime_ns


if __name__ == "__main__":
    unittest.main()
