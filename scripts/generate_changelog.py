#!/usr/bin/env python3
"""
Generate changelog for a specific version from CHANGELOG.md

This script extracts the changelog entry for a given version
and formats it for GitHub releases.
"""

import argparse
import re
from pathlib import Path


def extract_version_changelog(changelog_content, version):
    """Extract changelog entries for a specific version."""
    lines = changelog_content.split('\n')

    # Find the version header
    version_pattern = rf'^## \[?{re.escape(version)}\]? '
    in_version_section = False
    version_lines = []

    for line in lines:
        if re.match(version_pattern, line):
            in_version_section = True
            version_lines.append(line)
            continue
        elif in_version_section and line.startswith('## ['):
            # Next version section starts
            break
        elif in_version_section:
            version_lines.append(line)

    return '\n'.join(version_lines).strip()


def main():
    parser = argparse.ArgumentParser(description='Generate changelog for release')
    parser.add_argument('--version', required=True, help='Version to extract')
    parser.add_argument('--output', default='changelog.md', help='Output file')
    parser.add_argument('--changelog', default='../CHANGELOG.md', help='Changelog file path')

    args = parser.parse_args()

    # Read changelog
    changelog_path = Path(__file__).parent.parent / args.changelog
    if not changelog_path.exists():
        print(f"Error: Changelog file not found: {changelog_path}")
        return 1

    with open(changelog_path, 'r', encoding='utf-8') as f:
        changelog_content = f.read()

    # Extract version changelog
    version_changelog = extract_version_changelog(changelog_content, args.version)

    if not version_changelog:
        print(f"Warning: No changelog found for version {args.version}")
        version_changelog = f"## {args.version}\n\nRelease notes for version {args.version}."

    # Write output
    with open(args.output, 'w', encoding='utf-8') as f:
        f.write(version_changelog)

    print(f"Generated changelog for version {args.version} in {args.output}")
    return 0


if __name__ == '__main__':
    exit(main())
