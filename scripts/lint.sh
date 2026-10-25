#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
cd "$root"

for tool in editorconfig-checker zizmor; do
    if ! command -v "$tool" >/dev/null 2>&1; then
        echo "error: $tool is not installed" >&2
        exit 1
    fi
done

zizmor_pinned=$(sed -n 's/^zizmor==\([^ ]*\).*/\1/p' .github/requirements-lint.txt)
zizmor_local=$(zizmor --version | awk '{print $2}')
if [[ "$zizmor_local" != "$zizmor_pinned" ]]; then
    echo "warning: zizmor $zizmor_local installed, CI uses $zizmor_pinned" >&2
fi

run() {
    local output status=0
    output=$("$@" 2>&1) || status=$?
    if [[ -n "$output" ]]; then
        echo "-----"
        printf '%s\n' "$output"
        echo "-----"
    fi
    if [[ "$status" -eq 0 ]]; then
        echo "✓ $*"
    else
        echo "✗ $*"
    fi
    return "$status"
}

status=0
if [[ -n "${BASE_REF:-}" ]]; then
    diff_base=$BASE_REF
else
    diff_base=$(git merge-base HEAD origin/main)
fi
run git diff "$diff_base" --check || status=$?
run editorconfig-checker || status=$?
run zizmor --quiet --persona=auditor . || status=$?
exit "$status"
