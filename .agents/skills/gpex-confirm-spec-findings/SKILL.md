---
name: gpex-confirm-spec-findings
description: Validate gpex-review-specs findings one by one.
---

## Task

Validate gpex-review-specs accepted findings one by one.

## Workflow

1. If `./tmp/review-specs-findings-accepted.md` does not exist, stop and inform the user.
2. For each finding in `./tmp/review-specs-findings-accepted.md`:
    1. Display the finding details.
    2. Ask user which solution to apply (provide the finding solution and viable alternatives).
        1. If the finding is refused, move it to `./tmp/review-specs-findings-refused.md`.
        2. If the finding is accepted, update it in `./tmp/review-specs-findings-accepted.md` with
           the selected solution.
3. Stop and don't implement anything.

## Rules

- Always provide enough information, consider the user has not read the code nor the findings.
- Skip findings where the fix is necessary, obvious and without relevant alternative.
