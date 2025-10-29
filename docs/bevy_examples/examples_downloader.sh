#!/usr/bin/env bash
# Download only the `examples` directory from bevyengine/bevy@release-0.17.2
# into a plain directory (no .git, no history). No GitHub auth token required.
#
# Usage:
#   ./download-bevy-examples.sh [OUTDIR]
# Example:
#   ./download-bevy-examples.sh bevy-examples
#
# This script downloads a repository archive (zip or tar.gz) for the given ref,
# extracts it to a temporary directory, and copies the `examples/` tree into OUTDIR.
# Requirements: curl and either unzip or tar + cp (most systems have these).
set -euo pipefail

OUTDIR="${1:-bevy-examples}"
OWNER="bevyengine"
REPO="bevy"
REF="release-0.17.2"
TMPDIR="$(mktemp -d)"
ZIP="$TMPDIR/archive.zip"
TAR="$TMPDIR/archive.tar.gz"

cleanup() {
  rm -rf "$TMPDIR"
}
trap cleanup EXIT

# Check required commands
for cmd in curl mkdir; do
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "Error: required command '$cmd' not found." >&2
    exit 1
  fi
done

# Try to download a zip archive (tags, heads, fallback)
ZIP_URLS=(
  "https://github.com/${OWNER}/${REPO}/archive/refs/tags/${REF}.zip"
  "https://github.com/${OWNER}/${REPO}/archive/refs/heads/${REF}.zip"
  "https://github.com/${OWNER}/${REPO}/archive/${REF}.zip"
)

ARCHIVE_TYPE=""
for url in "${ZIP_URLS[@]}"; do
  echo "Trying: $url"
  if curl -f -L -o "$ZIP" "$url" >/dev/null 2>&1; then
    ARCHIVE_TYPE="zip"
    break
  fi
done

# If zip failed, try tar.gz
if [ -z "$ARCHIVE_TYPE" ]; then
  TAR_URLS=(
    "https://github.com/${OWNER}/${REPO}/archive/refs/tags/${REF}.tar.gz"
    "https://github.com/${OWNER}/${REPO}/archive/refs/heads/${REF}.tar.gz"
    "https://github.com/${OWNER}/${REPO}/archive/${REF}.tar.gz"
  )
  for url in "${TAR_URLS[@]}"; do
    echo "Trying: $url"
    if curl -f -L -o "$TAR" "$url" >/dev/null 2>&1; then
      ARCHIVE_TYPE="tar"
      break
    fi
  done
fi

if [ -z "$ARCHIVE_TYPE" ]; then
  echo "Failed to download archive for ${OWNER}/${REPO}@${REF} (no auth token used)." >&2
  echo "Please check that the ref exists and you have network access." >&2
  exit 1
fi

echo "Downloaded archive type: $ARCHIVE_TYPE"

# Extract archive
if [ "$ARCHIVE_TYPE" = "zip" ]; then
  if ! command -v unzip >/dev/null 2>&1; then
    echo "Error: unzip not found. Install unzip or ensure 'tar' + tar.gz archive is available." >&2
    exit 1
  fi
  unzip -q "$ZIP" -d "$TMPDIR"
else
  # tar.gz
  tar -xzf "$TAR" -C "$TMPDIR"
fi

# Find the extracted top-level directory (the repo-name-<ref> folder)
EXTRACTED_ROOT="$(find "$TMPDIR" -mindepth 1 -maxdepth 1 -type d | head -n 1)"
if [ -z "$EXTRACTED_ROOT" ] || [ ! -d "$EXTRACTED_ROOT" ]; then
  echo "Failed to find extracted repository root." >&2
  exit 1
fi

SRC_DIR="$EXTRACTED_ROOT/examples"
if [ ! -d "$SRC_DIR" ]; then
  echo "No examples directory found in the extracted archive at: $SRC_DIR" >&2
  exit 1
fi

# Copy examples to OUTDIR (preserve structure). Use rsync if available, otherwise cp -a.
mkdir -p "$OUTDIR"
if command -v rsync >/dev/null 2>&1; then
  rsync -a --delete "$SRC_DIR/" "$OUTDIR/"
else
  # cp -a preserves attributes; ensure target directory exists
  cp -a "$SRC_DIR/." "$OUTDIR/"
fi

echo "Done. examples/ from ${OWNER}/${REPO}@${REF} saved to: $OUTDIR"d