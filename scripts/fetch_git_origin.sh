#!/usr/bin/env bash
set -euo pipefail

zero=0000000000000000000000000000000000000000
if [ -n "$DIFF_BASE" ] && [ "$DIFF_BASE" != "$zero" ]; then
    git fetch --quiet origin "$DIFF_BASE"
    git rev-parse FETCH_HEAD
else
    git fetch --quiet origin main
    if [ "$(git rev-parse --is-shallow-repository)" = true ]; then
        git fetch --quiet --unshallow origin
        git fetch --quiet origin main
    fi
    git merge-base HEAD origin/main
fi
