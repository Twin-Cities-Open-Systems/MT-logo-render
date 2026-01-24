#!/usr/bin/env bash
set -euo pipefail

SRC_DIR="prompts/hee"
DST_DIR=".cursor/prompts"

mkdir -p "$DST_DIR"

rsync -a --delete \
  --exclude "docs/" \
  --exclude ".cursor/" \
  "$SRC_DIR/" "$DST_DIR/"

echo "Synced $SRC_DIR -> $DST_DIR"
