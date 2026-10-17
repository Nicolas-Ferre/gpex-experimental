#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
cd "$root"

if ! command -v editorconfig-checker >/dev/null 2>&1; then
    echo "error: editorconfig-checker is not installed" >&2
    exit 1
fi

run() {
    local output status=0
    output=$("$@" 2>&1) || status=$?
    if [[ -n "$output" ]]; then
        echo " -----"
        printf '%s\n' "$output" | sed 's/^/| /'
        echo " -----"
    fi
    if [[ "$status" -eq 0 ]]; then
        echo "✓ $*"
    else
        echo "✗ $*"
        return "$status"
    fi
}

run git diff FETCH_HEAD --check
run editorconfig-checker
