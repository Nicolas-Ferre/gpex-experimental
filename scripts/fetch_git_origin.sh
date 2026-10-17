#!/usr/bin/env bash
set -euo pipefail

zero=0000000000000000000000000000000000000000
if [ -n "$DIFF_BASE" ] && [ "$DIFF_BASE" != "$zero" ]; then
    git fetch origin "$DIFF_BASE"
else
    git fetch origin main
fi
