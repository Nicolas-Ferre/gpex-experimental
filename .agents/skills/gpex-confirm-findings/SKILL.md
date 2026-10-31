---
name: gpex-confirm-findings
description: Validate gpex-review-changes or gpex-review-specs findings one by one.
---

## Preconditions

- Never run the skill if not explicitly referenced by another skill or the user.

## Task

Validate accepted findings of a review one by one.

## Workflow

`<type>` is the skill argument: `changes` for `gpex-review-changes` findings, or `specs` for
`gpex-review-specs` findings. If it is missing, ask the user.

1. If `./tmp/review-<type>-findings-accepted.md` does not exist or has no finding, stop and inform
   the user.
2. For each finding in `./tmp/review-<type>-findings-accepted.md`:
    1. Read the finding location and the related code.
    2. Display the finding details.
    3. If the fix is necessary, obvious and without relevant alternative, keep the finding without
       asking. Else, ask user which solution to apply: the finding solution, viable alternatives,
       refusing the finding, or a solution they write themselves.
        1. If the finding is refused, ask for the reason unless the user already gave it, then move
           it to `./tmp/review-<type>-findings-refused.md` as a refused entry (see the finding
           format of `gpex-review-changes`, don't run this skill). When creating this file, start
           it with the following line, followed by an empty line:
            - `changes`: `Branch: <branch label>` (see `gpex-review-changes`, don't run this skill).
            - `specs`: `Specs: <specs path>` (see `gpex-review-specs`, don't run this skill), where
              the specs path is the file of the finding locations.
        2. If the finding is accepted, replace its `Fix:` and snippet in
           `./tmp/review-<type>-findings-accepted.md` with the selected solution.
3. List the findings kept without asking, so the user can still review them.
4. Stop and don't implement anything.

## Rules

- Always provide enough information, consider the user has not read the code nor the findings.
- Each question includes a recommended option.
- Take earlier decisions into account, and flag findings they make obsolete or conflicting.
