#!/usr/bin/env python3
"""
Update CHANGELOG.md after a release by moving unreleased changes to the new version.

This script takes the unreleased changes and creates a new version entry,
then adds a new "Unreleased" section for future changes.
"""

import argparse
import re
from datetime import datetime
from pathlib import Path


def update_changelog(changelog_path, version):
    """Update the changelog with the new version."""
    with open(changelog_path, 'r', encoding='utf-8') as f:
        content = f.read()

    # Find the unreleased section
    unreleased_pattern = r'## \[Unreleased\](.*?)(?=\n## \[|\n## [0-9]|\Z)'
    match = re.search(unreleased_pattern, content, re.DOTALL)

    if not match:
        print("No unreleased section found")
        return False

    unreleased_content = match.group(1).strip()

    # Generate new version entry
    today = datetime.now().strftime('%Y-%m-%d')
    new_version_entry = f'## [{version}] - {today}\n\n{unreleased_content}\n'

    # Replace unreleased section with new version
    updated_content = re.sub(
        unreleased_pattern,
        f'\n{new_version_entry}\n## [Unreleased] - Development Phase\n\n### Added\n\n### Changed\n\n### Fixed\n\n### Technical\n',
        content,
        flags=re.DOTALL
    )

    # Write back to file
    with open(changelog_path, 'w', encoding='utf-8') as f:
        f.write(updated_content)

    print(f"Updated CHANGELOG.md with version {version}")
    return True


def main():
    parser = argparse.ArgumentParser(description='Update changelog after release')
    parser.add_argument('--version', required=True, help='Released version')
    parser.add_argument('--changelog', default='../CHANGELOG.md', help='Changelog file path')

    args = parser.parse_args()

    changelog_path = Path(__file__).parent.parent / args.changelog
    if not changelog_path.exists():
        print(f"Error: Changelog file not found: {changelog_path}")
        return 1

    if update_changelog(changelog_path, args.version):
        return 0
    else:
        return 1


if __name__ == '__main__':
    exit(main())
