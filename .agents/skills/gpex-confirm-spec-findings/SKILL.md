---
name: gpex-confirm-spec-findings
description: Validate gpex-review-specs findings one by one.
---

## Preconditions

- Never run the skill if not explicitly referenced by another skill or the user.

## Task

Validate gpex-review-specs accepted findings one by one.

## Workflow

1. If `./tmp/review-specs-findings-accepted.md` does not exist or has no finding, stop and inform
   the user.
2. For each finding in `./tmp/review-specs-findings-accepted.md`:
    1. Read the specifications at the finding location and the related code.
    2. Display the finding details.
    3. Ask user which solution to apply: the finding solution, viable alternatives, refusing the
       finding, or a solution they write themselves.
        1. If the finding is refused, ask for the reason unless the user already gave it, then move
           it to `./tmp/review-specs-findings-refused.md` with a last `- Refusal: <reason>` bullet.
           When creating this file, start it with a `Specs: ./<finding location path>` line
           followed by an empty line.
        2. If the finding is accepted, replace its `Fix:` and snippet in
           `./tmp/review-specs-findings-accepted.md` with the selected solution.
3. Stop and don't implement anything.

## Rules

- Always provide enough information, consider the user has not read the code nor the findings.
- Each question includes a recommended option.
- Take earlier decisions into account, and flag findings they make obsolete or conflicting.
- Don't ask about findings where the fix is necessary, obvious and without relevant alternative:
  keep them accepted and list them at the end, so the user can still review them.
