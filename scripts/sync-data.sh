#!/bin/bash
# Sync JSON data files from the data repo.
# Usage: ./scripts/sync-data.sh [tag]
set -euo pipefail

TAG="${1:-main}"
REPO="onomojo/i18n-timezones-data"
DATA_DIR="$(cd "$(dirname "$0")/.." && pwd)/data"

echo "Syncing data from $REPO@$TAG..."

# Use git archive to fetch data/ directory
TMP=$(mktemp -d)
trap "rm -rf $TMP" EXIT

git archive --remote="git@github.com:$REPO.git" "$TAG" data/ | tar -x -C "$TMP" 2>/dev/null || {
    # Fallback: clone sparse
    git clone --depth 1 --branch "$TAG" --filter=blob:none --sparse "https://github.com/$REPO.git" "$TMP/repo"
    cd "$TMP/repo" && git sparse-checkout set data/
    cp "$TMP/repo/data/"*.json "$DATA_DIR/"
    echo "Done: $(ls "$DATA_DIR"/*.json | wc -l) files synced"
    exit 0
}

cp "$TMP/data/"*.json "$DATA_DIR/"
echo "Done: $(ls "$DATA_DIR"/*.json | wc -l) files synced"
