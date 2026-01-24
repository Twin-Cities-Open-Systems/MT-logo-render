#!/usr/bin/env bash
set -euo pipefail

./scripts/sync_cursor_prompts.sh

if ! git diff --quiet -- .cursor/prompts; then
  echo "ERROR: .cursor/prompts out of sync with prompts/hee"
  exit 1
fi
